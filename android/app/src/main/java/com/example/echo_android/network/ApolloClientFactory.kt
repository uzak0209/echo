package com.example.echo_android.network

import android.content.Context
import android.util.Log
import com.apollographql.apollo.ApolloClient
import com.apollographql.apollo.api.http.HttpRequest
import com.apollographql.apollo.api.http.HttpResponse
import com.apollographql.apollo.network.http.DefaultHttpEngine
import com.apollographql.apollo.network.http.HttpInterceptor
import com.apollographql.apollo.network.http.HttpInterceptorChain
import com.example.echo_android.repository.TokenRepository
import com.example.rocketreserver.RefreshTokenMutation
import kotlinx.coroutines.runBlocking
import okhttp3.OkHttpClient

lateinit var apolloClient: ApolloClient
    private set

// リフレッシュ専用のクライアント（インターセプターなしで循環参照を防ぐ）
private lateinit var refreshClient: ApolloClient

object ApolloClientFactory {
    private const val SERVER_URL = "http://10.0.2.2:8000/graphql"

    fun initialize(context: Context) {
        TokenRepository.init(context)

        // PersistentCookieJarを使用してCookieを永続化
        val cookieJar = PersistentCookieJar(context)

        // OkHttpClientにCookieJarを設定
        val okHttpClient = OkHttpClient.Builder()
            .cookieJar(cookieJar)
            .build()

        // リフレッシュ専用クライアント（インターセプターなし）
        refreshClient = ApolloClient.Builder()
            .serverUrl(SERVER_URL)
            .httpEngine(DefaultHttpEngine(okHttpClient))
            .build()

        apolloClient = ApolloClient.Builder()
            .serverUrl(SERVER_URL)
            .httpEngine(DefaultHttpEngine(okHttpClient))
            .addHttpInterceptor(AuthorizationInterceptor())
            .build()
        Log.d("ApolloClientFactory", "Initialized with PersistentCookieJar")
    }

    fun getRefreshClient(): ApolloClient = refreshClient
}

class AuthorizationInterceptor : HttpInterceptor {
    override suspend fun intercept(
        request: HttpRequest,
        chain: HttpInterceptorChain
    ): HttpResponse {
        val token = TokenRepository.getToken()
        Log.d("AuthInterceptor", "Token: ${if (token != null) "Present (${token.take(10)}...)" else "NULL"}")

        val newRequest = if (token != null) {
            request.newBuilder()
                .addHeader("Authorization", "Bearer $token")
                .build()
        } else {
            Log.w("AuthInterceptor", "No token available")
            request
        }

        var response = chain.proceed(newRequest)

        // Unauthorizedエラーの場合、トークンをリフレッシュしてリトライ
        if (response.statusCode == 401) {
            Log.d("AuthInterceptor", "401 Unauthorized detected, attempting token refresh")

            try {
                // リフレッシュトークンで新しいアクセストークンを取得
                val refreshResponse = runBlocking {
                    ApolloClientFactory.getRefreshClient()
                        .mutation(RefreshTokenMutation())
                        .execute()
                }

                if (refreshResponse.hasErrors()) {
                    Log.e("AuthInterceptor", "Token refresh failed: ${refreshResponse.errors}")
                    TokenRepository.removeToken() // トークンを削除してログアウト状態に
                    return response
                }

                val newAccessToken = refreshResponse.data?.refreshToken?.accessToken
                if (newAccessToken != null) {
                    Log.d("AuthInterceptor", "Token refreshed successfully")
                    TokenRepository.setToken(newAccessToken)

                    // 新しいトークンでリクエストをリトライ
                    val retryRequest = request.newBuilder()
                        .addHeader("Authorization", "Bearer $newAccessToken")
                        .build()
                    response = chain.proceed(retryRequest)
                    Log.d("AuthInterceptor", "Request retried with new token, status: ${response.statusCode}")
                } else {
                    Log.e("AuthInterceptor", "No access token in refresh response")
                    TokenRepository.removeToken()
                }
            } catch (e: Exception) {
                Log.e("AuthInterceptor", "Token refresh exception: ${e.message}", e)
                TokenRepository.removeToken()
            }
        }

        return response
    }
}

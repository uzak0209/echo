package com.example.echo_android.network

import android.content.Context
import android.util.Log
import com.apollographql.apollo.ApolloClient
import com.apollographql.apollo.api.http.HttpRequest
import com.apollographql.apollo.api.http.HttpResponse
import com.apollographql.apollo.network.http.HttpInterceptor
import com.apollographql.apollo.network.http.HttpInterceptorChain
import com.example.echo_android.repository.TokenRepository
import com.example.rocketreserver.RefreshTokenMutation

lateinit var apolloClient: ApolloClient
    private set

object ApolloClientFactory {
    private const val SERVER_URL = "http://10.0.2.2:8000/graphql"

    fun initialize(context: Context) {
        TokenRepository.init(context)
        apolloClient = ApolloClient.Builder()
            .serverUrl(SERVER_URL)
            .addHttpInterceptor(AuthorizationInterceptor())
            .build()
        Log.d("ApolloClientFactory", "Initialized")
    }
}

class AuthorizationInterceptor : HttpInterceptor {
    override suspend fun intercept(
        request: HttpRequest,
        chain: HttpInterceptorChain
    ): HttpResponse {
        val token = TokenRepository.getAccessToken()
        Log.d("AuthInterceptor", "Token: ${if (token != null) "Present (${token.take(10)}...)" else "NULL"}")

        val newRequest = if (token != null) {
            request.newBuilder()
                .addHeader("Authorization", "Bearer $token")
                .build()
        } else {
            Log.w("AuthInterceptor", "No token available")
            request
        }

        val response = chain.proceed(newRequest)

        if (response.statusCode == 401) {
            Log.w("AuthInterceptor", "Token expired(401)")

           val newAccessToken = refreshToken()
            if (newAccessToken != null) {
                TokenRepository.setToken(newAccessToken)
                val retriedRequest = request.newBuilder()
                    .addHeader("Authorization", "Bearer $newAccessToken")
                    .build()
                return chain.proceed(retriedRequest)
            } else {
                Log.e("AuthInterceptor", "Refresh token failed")
                TokenRepository.removeToken()
                return response
            }
        }

        return chain.proceed(newRequest)
    }

    private suspend fun refreshToken(): String? {
        return try {
            val response = apolloClient.mutation(RefreshTokenMutation()).execute()
            when {
                response.exception != null -> {
                    Log.e("AuthInterceptor", "refreshToken failed", response.exception)
                    null
                }
                response.hasErrors() -> {
                    Log.e("AuthInterceptor", "refreshToken GraphQL error: ${response.errors?.firstOrNull()?.message}")
                    null
                }
                else -> {
                    response.data?.refreshToken?.accessToken
                }
            }
        } catch (e: Exception) {
            Log.e("AuthInterceptor", "refreshToken error", e)
            null
        }
    }
}

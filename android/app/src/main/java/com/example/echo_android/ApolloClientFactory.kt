package com.example.echo_android

import android.content.Context
import android.util.Log
import com.apollographql.apollo.ApolloClient
import com.apollographql.apollo.api.http.HttpRequest
import com.apollographql.apollo.api.http.HttpResponse
import com.apollographql.apollo.network.http.HttpInterceptor
import com.apollographql.apollo.network.http.HttpInterceptorChain

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

        return chain.proceed(newRequest)
    }
}

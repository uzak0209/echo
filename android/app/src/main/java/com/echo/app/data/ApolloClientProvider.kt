package com.echo.app.data

import com.apollographql.apollo3.ApolloClient

object ApolloClientProvider {
    private const val SERVER_URL = "http://10.0.2.2:8000/graphql" // For Android emulator

    val apolloClient = ApolloClient.Builder()
        .serverUrl(SERVER_URL)
        .build()
}

package com.example.echo_android

import com.apollographql.apollo.ApolloClient

val apolloClient = ApolloClient.Builder()
    .serverUrl("http://10.0.2.2:8000/graphql")
    .build()
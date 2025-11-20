package com.example.echo_android.network

import com.apollographql.apollo.api.Error

class GraphQlServerException(errors: List<Error>) : Exception(errors.joinToString { it.message.toString() })
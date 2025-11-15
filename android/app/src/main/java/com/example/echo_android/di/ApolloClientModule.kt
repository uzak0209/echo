package com.example.echo_android.di

import com.apollographql.apollo.ApolloClient
import com.example.echo_android.network.ApolloWrapper
import com.example.echo_android.network.apolloClient
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton

@Module
@InstallIn(SingletonComponent::class)
object ApolloClientModule {

    @Provides
    @Singleton
    fun provideApolloClient(): ApolloClient = apolloClient

    @Provides
    @Singleton
    fun provideApolloWrapper(client: ApolloClient): ApolloWrapper {
        return ApolloWrapper(client)
    }
}
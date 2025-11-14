package com.example.echo_android

import android.content.Context
import com.apollographql.apollo.ApolloClient
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
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
package com.example.echo_android.network

import com.apollographql.apollo.ApolloClient
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.LoginMutation
import com.example.rocketreserver.SignupMutation
import android.util.Log
import com.apollographql.apollo.api.Optional
import com.example.rocketreserver.CreatePostMutation

class ApolloWrapper(
    private val client: ApolloClient
) {
    suspend fun getTimeline(): Result<List<GetTimelineQuery.Timeline>> {
        return try {
            val response = client.query(GetTimelineQuery()).execute()
            val posts = response.data?.timeline ?: emptyList()
            Result.success(posts)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun login(username: String, password: String): String?   {
        val response = client.mutation(LoginMutation(username = username, password = password)).execute()

        if (response.exception != null) return null
        if (response.hasErrors()) return null

        return response.data?.login?.accessToken
    }

    suspend fun signup(username: String, password: String): String? {
        val response = client.mutation(SignupMutation(username = username, password = password)).execute()

        if (response.exception != null) return null
        if (response.hasErrors()) return null

        return response.data?.signup?.accessToken
    }

    suspend fun createPost(content: String, imageUrl: String?): Boolean {
        val wrappedImageUrl = if (imageUrl != null) {
            Optional.present(imageUrl)
        } else {
            Optional.absent()
        }

        return try {
            val response = client.mutation(
                CreatePostMutation(
                    content = content,
                    imageUrl = wrappedImageUrl
                )
            ).execute()

            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "createPost failed", response.exception)
                    false
                }
                response.hasErrors() -> {
                    Log.e(
                        "ApolloWrapper",
                        "createPost GraphQL error: ${response.errors?.firstOrNull()?.message}"
                    )
                    false
                }
                else -> {
                    Log.d("ApolloWrapper", "createPost success")
                    true // 成功！
                }
            }

        } catch (e: Exception) {
            Log.e("ApolloWrapper", "createPost error", e)
            false
        }
    }
}
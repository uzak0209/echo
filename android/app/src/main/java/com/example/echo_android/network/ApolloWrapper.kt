package com.example.echo_android.network

import com.apollographql.apollo.ApolloClient
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.LoginMutation
import com.example.rocketreserver.SignupMutation
import android.util.Log
import com.apollographql.apollo.api.Optional
import com.example.rocketreserver.AddReactionMutation
import com.example.rocketreserver.CreatePostMutation
import com.example.rocketreserver.GenerateSseTokenMutation
import com.example.rocketreserver.RemoveReactionMutation
import com.example.rocketreserver.type.ReactionTypeGql
import kotlinx.coroutines.flow.Flow

class ApolloWrapper(
    private val client: ApolloClient
) {
    fun fetchTimeline(): Flow<GetTimelineQuery.Data> {
        return client.query(GetTimelineQuery())
            .toThrowableFlow()
    }

    suspend fun generateSseToken(): String? {
        val response = client.mutation(GenerateSseTokenMutation()).execute()

        return try {
            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "generateSseToken failed", response.exception)
                    return null
                }
                response.hasErrors() -> {
                    Log.e("ApolloWrapper", "generateSseToken GraphQL error: ${response.errors?.firstOrNull()?.message}")
                    return null
                }
                else -> {
                    Log.d("ApolloWrapper", "generateSseToken success")
                    response.data?.generateSseToken
                }
            }
        } catch (e: Exception) {
            Log.e("ApolloWrapper", "generateSseToken error", e)
            null
        }
    }

    suspend fun login(username: String, password: String): String?   {
        return try {
            val response = client.mutation(
                LoginMutation(username = username, password = password)
            ).execute()

            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "login failed", response.exception)
                    null
                }
                response.hasErrors() -> {
                    Log.e("ApolloWrapper", "login GraphQL error: ${response.errors?.firstOrNull()?.message}")
                    null
                }
                else -> {
                    Log.d("ApolloWrapper", "login success")
                    response.data?.login?.accessToken
                }
            }
        } catch (e: Exception) {
            Log.e("ApolloWrapper", "login error", e)
            null
        }
    }

    suspend fun signup(username: String, password: String): String? {
        return try {
            val response = client.mutation(
                SignupMutation(username = username, password = password)
            ).execute()

            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "signup failed", response.exception)
                    null
                }
                response.hasErrors() -> {
                    Log.e("ApolloWrapper", "signup GraphQL error: ${response.errors?.firstOrNull()?.message}")
                    null
                }
                else -> {
                    Log.d("ApolloWrapper", "signup success")
                    response.data?.signup?.accessToken
                }
            }
        } catch (e: Exception) {
            Log.e("ApolloWrapper", "signup error", e)
            null
        }
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

    suspend fun addReaction(postId: String, typeGql: ReactionTypeGql): Boolean {
        return try {
            val response = client.mutation(AddReactionMutation(postId, typeGql)).execute()

            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "addReaction failed", response.exception)
                    false
                }
                response.hasErrors() -> {
                    Log.e(
                        "ApolloWrapper",
                        "addReaction GraphQL error: ${response.errors?.firstOrNull()?.message}"
                    )
                    false
                }
                else -> {
                    Log.d("ApolloWrapper", "addReaction success")
                    true
                }
            }
        } catch (e: Exception) {
            Log.e("ApolloWrapper", "addReaction error", e)
            false
        }
    }

    suspend fun removeReaction(postId: String): Boolean {
        return try {
            val response = client.mutation(RemoveReactionMutation(postId)).execute()

            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "removeReaction failed", response.exception)
                    false
                }
                response.hasErrors() -> {
                    Log.e(
                        "ApolloWrapper",
                        "removeReaction GraphQL error: ${response.errors?.firstOrNull()?.message}"
                    )
                    false
                }
                else -> {
                    Log.d("ApolloWrapper", "removeReaction success")
                    true
                }
            }
        } catch (e: Exception) {
            Log.e("ApolloWrapper", "removeReaction error", e)
            false
        }
    }
}
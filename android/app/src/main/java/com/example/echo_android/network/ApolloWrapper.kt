package com.example.echo_android.network

import com.apollographql.apollo.ApolloClient
import com.example.rocketreserver.GetTimelineQuery
import com.example.rocketreserver.LoginMutation
import com.example.rocketreserver.SignupMutation
import com.example.rocketreserver.RefreshTokenMutation
import android.util.Log
import com.apollographql.apollo.api.Optional
import com.apollographql.apollo.api.ApolloResponse
import com.apollographql.apollo.api.Mutation
import com.example.echo_android.repository.TokenRepository
import com.example.rocketreserver.AddReactionMutation
import com.example.rocketreserver.CreatePostMutation
import com.example.rocketreserver.GenerateSseTokenMutation
import com.example.rocketreserver.RemoveReactionMutation
import com.example.rocketreserver.type.ReactionTypeGql
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.retryWhen

class ApolloWrapper(
    private val client: ApolloClient
) {
    // Unauthorizedエラーかどうかをチェック
    private fun isUnauthorizedError(response: ApolloResponse<*>): Boolean {
        return response.errors?.any { error ->
            error.message.contains("Unauthorized", ignoreCase = true) ||
            error.message.contains("No valid access token", ignoreCase = true)
        } == true
    }

    // トークンをリフレッシュしてミューテーションをリトライ
    private suspend fun <D : Mutation.Data> executeMutationWithRetry(
        mutation: Mutation<D>
    ): ApolloResponse<D> {
        var response = client.mutation(mutation).execute()

        // Unauthorizedエラーの場合、トークンをリフレッシュしてリトライ
        if (isUnauthorizedError(response)) {
            Log.d("ApolloWrapper", "Unauthorized error detected, attempting token refresh")

            val newToken = refreshToken()
            if (newToken != null) {
                TokenRepository.setToken(newToken)
                Log.d("ApolloWrapper", "Token refreshed, retrying mutation")
                response = client.mutation(mutation).execute()
            } else {
                Log.e("ApolloWrapper", "Token refresh failed")
                TokenRepository.removeToken()
            }
        }

        return response
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

    fun fetchTimeline(): Flow<GetTimelineQuery.Data> {
        return client.query(GetTimelineQuery())
            .toFlow()
            .map { response ->
                if (response.exception != null) {
                    throw response.exception!!
                }
                if (response.hasErrors()) {
                    val errorMessage = response.errors?.firstOrNull()?.message ?: "Unknown error"
                    // Unauthorizedエラーの場合はリトライフラグを立てる
                    if (isUnauthorizedError(errorMessage)) {
                        throw UnauthorizedException(errorMessage)
                    }
                    throw Exception("GraphQL error: $errorMessage")
                }
                response.data ?: throw Exception("No data in response")
            }
            .retryWhen { cause, attempt ->
                if (cause is UnauthorizedException && attempt < 1) {
                    Log.d("ApolloWrapper", "fetchTimeline Unauthorized, attempting token refresh")
                    val newToken = refreshToken()
                    if (newToken != null) {
                        TokenRepository.setToken(newToken)
                        Log.d("ApolloWrapper", "Token refreshed, retrying fetchTimeline")
                        true // リトライする
                    } else {
                        Log.e("ApolloWrapper", "Token refresh failed")
                        TokenRepository.removeToken()
                        false // リトライしない
                    }
                } else {
                    false // リトライしない
                }
            }
    }

    private fun isUnauthorizedError(message: String): Boolean {
        return message.contains("Unauthorized", ignoreCase = true) ||
                message.contains("No valid access token", ignoreCase = true)
    }

    private class UnauthorizedException(message: String) : Exception(message)

    suspend fun login(username: String, password: String): String?   {
        // ログインは認証不要なのでrefreshClientを使用（インターセプターなし）
        val response = ApolloClientFactory.getRefreshClient()
            .mutation(LoginMutation(username = username, password = password))
            .execute()

        if (response.exception != null) return null
        if (response.hasErrors()) return null

        return response.data?.login?.accessToken
    }

    suspend fun signup(username: String, password: String): String? {
        // サインアップは認証不要なのでrefreshClientを使用（インターセプターなし）
        val response = ApolloClientFactory.getRefreshClient()
            .mutation(SignupMutation(username = username, password = password))
            .execute()

        if (response.exception != null) return null
        if (response.hasErrors()) return null

        return response.data?.signup?.accessToken
    }

    suspend fun refreshToken(): String? {
        return try {
            // リフレッシュ専用クライアントを使用（循環参照を防ぐ）
            val refreshClient = ApolloClientFactory.getRefreshClient()
            val response = refreshClient.mutation(RefreshTokenMutation()).execute()

            when {
                response.exception != null -> {
                    Log.e("ApolloWrapper", "refreshToken failed", response.exception)
                    null
                }
                response.hasErrors() -> {
                    Log.e("ApolloWrapper", "refreshToken GraphQL error: ${response.errors?.firstOrNull()?.message}")
                    null
                }
                else -> {
                    Log.d("ApolloWrapper", "refreshToken success")
                    response.data?.refreshToken?.accessToken
                }
            }
        } catch (e: Exception) {
            Log.e("ApolloWrapper", "refreshToken error", e)
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
            val mutation = CreatePostMutation(
                content = content,
                imageUrl = wrappedImageUrl
            )
            val response = executeMutationWithRetry(mutation)

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
            val mutation = AddReactionMutation(postId, typeGql)
            val response = executeMutationWithRetry(mutation)

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
            val mutation = RemoveReactionMutation(postId)
            val response = executeMutationWithRetry(mutation)

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
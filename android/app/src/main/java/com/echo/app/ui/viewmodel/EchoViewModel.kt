package com.echo.app.ui.viewmodel

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.apollographql.apollo3.exception.ApolloException
import com.echo.app.CreatePostMutation
import com.echo.app.GetTimelineQuery
import com.echo.app.IncrementDisplayCountMutation
import com.echo.app.data.ApolloClientProvider
import kotlinx.coroutines.launch

data class Post(
    val id: String,
    val content: String,
    val imageUrl: String?
)

class EchoViewModel : ViewModel() {
    var posts by mutableStateOf<List<Post>>(emptyList())
        private set

    var isLoading by mutableStateOf(false)
        private set

    fun loadTimeline() {
        viewModelScope.launch {
            isLoading = true
            try {
                val response = ApolloClientProvider.apolloClient
                    .query(GetTimelineQuery(limit = 10))
                    .execute()

                posts = response.data?.timeline?.map { post ->
                    Post(
                        id = post.id,
                        content = post.content,
                        imageUrl = post.imageUrl
                    )
                } ?: emptyList()
            } catch (e: ApolloException) {
                e.printStackTrace()
            } finally {
                isLoading = false
            }
        }
    }

    fun createPost(content: String) {
        viewModelScope.launch {
            try {
                ApolloClientProvider.apolloClient
                    .mutation(CreatePostMutation(content = content, imageUrl = null))
                    .execute()

                loadTimeline()
            } catch (e: ApolloException) {
                e.printStackTrace()
            }
        }
    }

    fun incrementDisplayCount(postId: String) {
        viewModelScope.launch {
            try {
                ApolloClientProvider.apolloClient
                    .mutation(IncrementDisplayCountMutation(postId = postId.toInt()))
                    .execute()
            } catch (e: ApolloException) {
                e.printStackTrace()
            }
        }
    }
}

package com.example.echo_android.ui.feature.post

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.echo_android.network.ApolloWrapper
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class CreatePostViewModel @Inject constructor(
    private val apollo: ApolloWrapper
): ViewModel() {
    private  val _state = MutableStateFlow(PostState())
    val state = _state.asStateFlow()

    fun createPost(content: String, imageUrl: String?) {
        viewModelScope.launch {
            _state.value = _state.value.copy(isLoading = true)
            val success = apollo.createPost(content, imageUrl)

            if (success) {
                Log.d("CreatePostViewModel", "Success to create post")
                _state.value = PostState(isLoading = false, posted = true)
            } else {
                Log.d("CreatePostViewModel", "Failed to create post")
                _state.value = PostState(isLoading = false, posted = false, error = "投稿に失敗しました")
            }
        }
    }

    fun resetState() {
        Log.d("CreatePostViewModel", "Resetting state")
        _state.value = PostState()
    }

    data class PostState(
        val isLoading: Boolean = false,
        val posted: Boolean = false,
        val error: String? = null
    )
}
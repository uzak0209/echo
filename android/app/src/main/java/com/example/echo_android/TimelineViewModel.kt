package com.example.echo_android

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.rocketreserver.GetTimelineQuery
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class TimelineViewModel @Inject constructor(
    private val apolloWrapper: ApolloWrapper
) : ViewModel() {

    private val _viewState = MutableStateFlow<List<GetTimelineQuery.Timeline>>(emptyList())
    val viewState = _viewState.asStateFlow()
    fun getTimeline() {
        viewModelScope.launch {
            Log.d("TimelineViewModel", "getTimeline called")
            val result = apolloWrapper.getTimeline()
            result.onSuccess { posts ->
                Log.d("TimelineViewModel", "getTimeline success: ${posts.size} posts")
                _viewState.value = posts
            }
        }
    }
}
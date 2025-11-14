package com.example.echo_android

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class LoginViewModel @Inject constructor(
    private  val apollo: ApolloWrapper
): ViewModel() {
    private  val _state = MutableStateFlow(false)
    val state = _state.asStateFlow()

    fun login(username: String, password: String) {
        viewModelScope.launch {
            val token = apollo.login(username, password)
            if (token != null) {
                TokenRepository.setToken(token)
                _state.value = true
            } else {
                _state.value = false
            }
        }
    }
}
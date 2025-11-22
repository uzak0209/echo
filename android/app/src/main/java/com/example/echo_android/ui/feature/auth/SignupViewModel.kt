package com.example.echo_android.ui.feature.auth

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.echo_android.network.ApolloWrapper
import com.example.echo_android.repository.TokenRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class SignupViewModel @Inject constructor(
    private val apollo: ApolloWrapper
): ViewModel(){
    private val _state = MutableStateFlow<AuthResult>(AuthResult.Idle)
    val state = _state.asStateFlow()

    fun signup(username: String, password: String) {
        viewModelScope.launch {
            _state.value = AuthResult.Loading
            val token = apollo.signup(username, password)
            if (token != null) {
                TokenRepository.setToken(token)
                _state.value = AuthResult.Success
            } else {
                _state.value = AuthResult.Error("ログインに失敗しました。ユーザー名が既に使用されている可能性があります。")
            }
        }
    }

    sealed class AuthResult {
        data object Idle : AuthResult()
        data object Loading : AuthResult()
        data object Success : AuthResult()
        data class Error(val message: String) : AuthResult()
    }
}
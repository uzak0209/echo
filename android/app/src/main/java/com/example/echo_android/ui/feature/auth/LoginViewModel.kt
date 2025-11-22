package com.example.echo_android.ui.feature.auth

import android.util.Log
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
class LoginViewModel @Inject constructor(
    private  val apollo: ApolloWrapper
): ViewModel() {
    private  val _state = MutableStateFlow<AuthResult>(AuthResult.Idle)
    val state = _state.asStateFlow()

    fun login(username: String, password: String) {
        viewModelScope.launch {
            _state.value = AuthResult.Loading
            val token = apollo.login(username, password)
            if (token != null) {
                TokenRepository.setToken(token)
                Log.d("LoginViewModel", "Success to login")
                _state.value = AuthResult.Success
            } else {
                Log.d("LoginViewModel", "Failed to login")
                _state.value = AuthResult.Error("アカウントの作成に失敗しました。ユーザー名またはパスワードが正しくありません。")
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
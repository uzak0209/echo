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
class AuthViewModel @Inject constructor(
    private val apolloWrapper: ApolloWrapper
) : ViewModel() {

    private val _authState = MutableStateFlow<AuthState>(AuthState.Checking)
    val authState = _authState.asStateFlow()

    init {
        checkAuthentication()
    }

    /**
     * アプリ起動時に認証状態をチェック
     * 1. 既存のアクセストークンがあればそれを使用
     * 2. なければrefresh tokenで新しいアクセストークンを取得
     * 3. どちらも失敗すれば未認証状態
     */
    private fun checkAuthentication() {
        viewModelScope.launch {
            _authState.value = AuthState.Checking
            Log.d("AuthViewModel", "Checking authentication...")

            // 既存のアクセストークンをチェック
            val existingToken = TokenRepository.getToken()
            if (existingToken != null) {
                Log.d("AuthViewModel", "Existing access token found")
                _authState.value = AuthState.Authenticated
                return@launch
            }

            // refresh tokenで新しいアクセストークンを取得
            Log.d("AuthViewModel", "Trying to refresh token...")
            val newToken = apolloWrapper.refreshToken()
            if (newToken != null) {
                Log.d("AuthViewModel", "Token refresh successful")
                TokenRepository.setToken(newToken)
                _authState.value = AuthState.Authenticated
            } else {
                Log.d("AuthViewModel", "Token refresh failed - user needs to login")
                _authState.value = AuthState.Unauthenticated
            }
        }
    }

    /**
     * 認証状態を再チェック（ログイン後など）
     */
    fun recheckAuthentication() {
        checkAuthentication()
    }

    /**
     * ログアウト処理
     */
    fun logout() {
        Log.d("AuthViewModel", "Logging out...")
        TokenRepository.removeToken()
        _authState.value = AuthState.Unauthenticated
    }

    sealed class AuthState {
        data object Checking : AuthState()
        data object Authenticated : AuthState()
        data object Unauthenticated : AuthState()
    }
}

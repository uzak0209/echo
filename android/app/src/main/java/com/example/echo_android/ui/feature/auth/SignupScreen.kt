package com.example.echo_android.ui.feature.auth

import android.util.Log
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.hilt.navigation.compose.hiltViewModel
import com.example.echo_android.repository.TokenRepository
import com.example.echo_android.network.apolloClient
import com.example.rocketreserver.SignupMutation

@Composable
fun SignupScreen(
    navigateBack: () -> Unit,
    onSecondaryClick: () -> Unit,
    viewModel: SignupViewModel = hiltViewModel()
) {
    val authResult by viewModel.state.collectAsState()
    var errorMessage by remember { mutableStateOf<String?>(null) }

    LaunchedEffect(authResult) {
        when (authResult) {
            is SignupViewModel.AuthResult.Success -> navigateBack()
            is SignupViewModel.AuthResult.Error -> {
                errorMessage = (authResult as SignupViewModel.AuthResult.Error).message
            }
            else -> {}
        }
    }

    AuthScreen(
        title = "Signup",
        buttonText = "登録",
        onAuthClick = { username, password ->
            viewModel.signup(username, password)
            true
        },
        onSecondaryClick = { onSecondaryClick() },
        secondaryText = "ログインはこちら",
        externalError = errorMessage
    )
}
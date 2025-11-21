package com.example.echo_android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeDrawing
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.example.echo_android.network.ApolloClientFactory
import com.example.echo_android.ui.feature.MainScreen
import com.example.echo_android.ui.feature.auth.AuthViewModel
import com.example.echo_android.ui.feature.auth.LoginScreen
import com.example.echo_android.ui.feature.auth.SignupScreen
import com.example.echo_android.ui.navigation.NavigationDestinations
import com.example.echo_android.ui.theme.EchoandroidTheme
import dagger.hilt.android.AndroidEntryPoint

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        ApolloClientFactory.initialize(this)
        enableEdgeToEdge()
        setContent {
            EchoandroidTheme {
                Scaffold(modifier = Modifier.fillMaxSize().windowInsetsPadding(WindowInsets.safeDrawing)) { innerPadding ->
                    MainNavHost()
                }
            }
        }
    }
}

@Composable
private fun MainNavHost() {
    val authViewModel: AuthViewModel = hiltViewModel()
    val authState by authViewModel.authState.collectAsState()

    // 認証状態チェック中はローディング表示
    when (authState) {
        is AuthViewModel.AuthState.Checking -> {
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                CircularProgressIndicator()
            }
        }
        is AuthViewModel.AuthState.Authenticated -> {
            AuthenticatedNavHost(onLogout = { authViewModel.logout() })
        }
        is AuthViewModel.AuthState.Unauthenticated -> {
            UnauthenticatedNavHost()
        }
    }
}

@Composable
private fun UnauthenticatedNavHost() {
    val navController = rememberNavController()
    NavHost(navController, startDestination = NavigationDestinations.LOGIN) {
        composable(route = NavigationDestinations.LOGIN) {
            LoginScreen(
                navigateBack = {
                    navController.navigate(NavigationDestinations.HOME) {
                        popUpTo(NavigationDestinations.LOGIN) { inclusive = true }
                    }
                },
                onSecondaryClick = {
                    navController.navigate(NavigationDestinations.SIGNUP)
                }
            )
        }

        composable(route = NavigationDestinations.SIGNUP) {
            SignupScreen(
                navigateBack = {
                    navController.navigate(NavigationDestinations.HOME) {
                        popUpTo(NavigationDestinations.SIGNUP) { inclusive = true }
                    }
                },
                onSecondaryClick = {
                    navController.navigate(NavigationDestinations.LOGIN)
                }
            )
        }

        composable(route = NavigationDestinations.HOME) {
            MainScreen()
        }
    }
}

@Composable
private fun AuthenticatedNavHost(onLogout: () -> Unit) {
    val navController = rememberNavController()
    NavHost(navController, startDestination = NavigationDestinations.HOME) {
        composable(route = NavigationDestinations.HOME) {
            MainScreen(onLogout = onLogout)
        }

        composable(route = NavigationDestinations.LOGIN) {
            LoginScreen(
                navigateBack = {
                    navController.navigate(NavigationDestinations.HOME) {
                        popUpTo(NavigationDestinations.LOGIN) { inclusive = true }
                    }
                },
                onSecondaryClick = {
                    navController.navigate(NavigationDestinations.SIGNUP)
                }
            )
        }

        composable(route = NavigationDestinations.SIGNUP) {
            SignupScreen(
                navigateBack = {
                    navController.navigate(NavigationDestinations.HOME) {
                        popUpTo(NavigationDestinations.SIGNUP) { inclusive = true }
                    }
                },
                onSecondaryClick = {
                    navController.navigate(NavigationDestinations.LOGIN)
                }
            )
        }
    }
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    EchoandroidTheme {
    }
}
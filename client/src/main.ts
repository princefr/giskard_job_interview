import './assets/main.css'

import { createApp, provide, h  } from 'vue'
import { createPinia } from 'pinia'
import { createApolloProvider } from '@vue/apollo-option';
import apolloClient from './apollo-client'
import { DefaultApolloClient } from '@vue/apollo-composable'

import App from './App.vue'
import router from './router'

const apolloProvider = createApolloProvider({
    defaultClient: apolloClient
})

const app = createApp(
    {
        setup() {
            provide(DefaultApolloClient, apolloClient)
        },
        render: () => h(App)
    }
)


app.use(createPinia())
app.use(router)


app.mount('#app')

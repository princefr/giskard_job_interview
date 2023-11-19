import {ApolloClient, createHttpLink, InMemoryCache, gql} from '@apollo/client/core';


// HTTP connection to the API
const httpLink = createHttpLink({
  uri: 'http://localhost:8000/',
});


const cache = new InMemoryCache();


const apolloClient = new ApolloClient({
  link: httpLink,
  cache,
});


export default apolloClient
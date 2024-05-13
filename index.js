const { ApolloServer } = require('apollo-server-express');
const { typeDefs } = require('./schema.graphql');
const { resolvers } = require('./resolvers');

const server = new ApolloServer({ typeDefs, resolvers });

server.applyMiddleware({ app: app });

// Start the server
server.listen({ port: 4000 }).then(({ url }) => {
  console.log(`🚀  Server ready at ${url}`);
});
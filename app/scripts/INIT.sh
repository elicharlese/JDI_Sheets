#!/bin/bash

# Set the directory name
directory_name="new_project"

# Create the project directory
mkdir $directory_name
cd $directory_name

# Initialize a new Git repository
git init

# Create a README.md file
touch README.md

# Initialize the data store for GraphQL
npm install --save-dev apollo-server-express graphql

# Create a new GraphQL schema file
touch schema.graphql

# Create a new GraphQL server file
touch index.js

# Add the following code to the index.js file:
```javascript
const { ApolloServer } = require('apollo-server-express');
const { typeDefs } = require('./schema.graphql');
const { resolvers } = require('./resolvers');

const server = new ApolloServer({ typeDefs, resolvers });

server.applyMiddleware({ app: app });

// Start the server
server.listen({ port: 4000 }).then(({ url }) => {
  console.log(`🚀  Server ready at ${url}`);
});
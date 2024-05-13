# graphql/Dockerfile

# Use a Node.js base image
FROM node:latest

# Create and set the working directory
WORKDIR /usr/src/app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application code
COPY . .

# Expose the port the app runs on
EXPOSE 4000

# Run the app
CMD [ "node", "src/index.js" ]
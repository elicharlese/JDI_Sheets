// JavaScript client
const query = gql`
  query Products($marketplaceSlug: String!, $productLineSlug: String!) {
    products(marketplaceSlug: $marketplaceSlug, productLineSlug: $productLineSlug) {
      id
      name
      variants {
        id
        name
        inventoryQuantity
      }
    }
  }
`;

const { Pool } = require('pg');
const pool = new Pool({
  connectionString: process.env.DATABASE_URL
});

module.exports = {
  query: (text, params) => pool.query(text, params),
};

const variables = {
  marketplaceSlug: "shopify",
  productLineSlug: "clothing",
};

fetch('/graphql', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({ query, variables }),
})
  .then(response => response.json())
  .then(data => console.log(data));
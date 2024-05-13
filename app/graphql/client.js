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
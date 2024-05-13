
#[derive(Debug)]
pub struct Marketplaces {
    amazon: Vec<Amazon>,
    shopify: Vec<Shopify>,
    walmart: Vec<Walmart>,
    michaels: Vec<Michaels>,
    etsy: Vec<Etsy>,
    ebay: Vec<Ebay>,
}

#[derive(Debug)]
pub struct Amazon {
    // ...
}

#[derive(Debug)]
pub struct Shopify {
    // ...
}

#[derive(Debug)]
pub struct Walmart {
    // ...
}

#[derive(Debug)]
pub struct Michaels {
    // ...
}

#[derive(Debug)]
pub struct Etsy {
    // ...
}

#[derive(Debug)]
pub struct Ebay {
    // ...
}

#[derive(Debug)]
pub struct ProductLines {
    brew_glitter: Vec<BrewGlitter>,
    tinker_dust: Vec<TinkerDust>,
    luster_dust: Vec<LusterDust>,
}

#[derive(Debug)]
pub struct BrewGlitter {
    // ...
}

#[derive(Debug)]
pub struct TinkerDust {
    // ...
}

#[derive(Debug)]
pub struct LusterDust {
    // ...
}

#[derive(Debug)]
pub struct Products {
    // ...
}

-- Your SQL goes here
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image VARCHAR(255) NOT NULL
);
CREATE TABLE companies (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    original_price NUMERIC(10, 2) NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    description TEXT NOT NULL,
    stock INT NOT NULL DEFAULT 0,
    colors JSONB NOT NULL,
    sizes JSONB NOT NULL,
    images JSONB NOT NULL,
    rating NUMERIC(2, 1) NOT NULL DEFAULT 0,
    company_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (company_id) REFERENCES companies(id)
);
CREATE TABLE product_categories (
    product_id INT NOT NULL,
    category_id INT NOT NULL,
    PRIMARY KEY (product_id, category_id),
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);
SET NAMES utf8mb4;

CREATE DATABASE IF NOT EXISTS clientes_rust_db
CHARACTER SET utf8mb4
COLLATE utf8mb4_unicode_ci;

USE clientes_rust_db;

SET NAMES utf8mb4;

CREATE TABLE IF NOT EXISTS clientes (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(150) NOT NULL,
    cpf VARCHAR(14) NOT NULL
);

INSERT INTO clientes (nome, cpf) VALUES
('José Herminio', '123.456.789-09'),
('Elias Mendes', '987.654.321-00'),
('Zubi Seru', '456.789.123-45'),
('Cassandra Cain', '321.654.987-78'),
('Zatanna Zatara', '789.123.456-90'),
('Bruce Wayne', '111.222.333-44'),
('Diana Themyscira', '555.666.777-88'),
('Clark Kent', '999.888.777-66');
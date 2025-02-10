-- Migrations will appear here as you chat with AI

create table branches (
  id bigint primary key generated always as identity,
  name text not null,
  location text not null
);

create table stores (
  id bigint primary key generated always as identity,
  name text not null,
  location text not null,
  branch_id bigint references branches (id)
);

create table products (
  id bigint primary key generated always as identity,
  name text not null,
  category text not null,
  price numeric(10, 2) not null,
  store_id bigint references stores (id)
);

create table delivery_personnel (
  id bigint primary key generated always as identity,
  name text not null,
  contact_number text not null,
  availability boolean not null
);

create table orders (
  id bigint primary key generated always as identity,
  customer_name text not null,
  customer_contact text not null,
  order_date timestamp with time zone default now(),
  delivery_personnel_id bigint references delivery_personnel (id),
  store_id bigint references stores (id)
);

create table order_items (
  id bigint primary key generated always as identity,
  order_id bigint references orders (id),
  product_id bigint references products (id),
  quantity int not null
);

create table shipping (
  id bigint primary key generated always as identity,
  order_id bigint references orders (id),
  shipping_status text not null,
  shipping_time timestamp with time zone
);

alter table orders
add column order_type text not null;

create table scheduled_deliveries (
  id bigint primary key generated always as identity,
  order_id bigint references orders (id),
  scheduled_time timestamp with time zone not null,
  status text not null
);

create table services (
  id bigint primary key generated always as identity,
  name text not null,
  description text,
  price numeric(10, 2) not null
);

create table order_services (
  id bigint primary key generated always as identity,
  order_id bigint references orders (id),
  service_id bigint references services (id),
  quantity int not null
);

drop table if exists order_services;

drop table if exists services;

alter table products
drop category;

create table product_categories (
  id bigint primary key generated always as identity,
  product_id bigint references products (id),
  category text not null
);

create table categorias (
  id bigint primary key generated always as identity,
  name text not null unique
);

alter table product_categories
drop category;

alter table product_categories
add column category_id bigint references categorias (id);

alter table branches
rename to sucursales;

alter table stores
rename to tiendas;

alter table products
rename to productos;

alter table delivery_personnel
rename to personal_entrega;

alter table orders
rename to pedidos;

alter table order_items
rename to items_pedido;

alter table shipping
rename to envios;

alter table scheduled_deliveries
rename to entregas_programadas;

alter table product_categories
rename to categorias_producto;

alter table sucursales
rename column name to nombre;

alter table sucursales
rename column location to ubicacion;

alter table tiendas
rename column name to nombre;

alter table tiendas
rename column location to ubicacion;

alter table tiendas
rename column branch_id to sucursal_id;

alter table productos
rename column name to nombre;

alter table productos
rename column price to precio;

alter table productos
rename column store_id to tienda_id;

alter table personal_entrega
rename column name to nombre;

alter table personal_entrega
rename column contact_number to numero_contacto;

alter table personal_entrega
rename column availability to disponibilidad;

alter table pedidos
rename column customer_name to nombre_cliente;

alter table pedidos
rename column customer_contact to contacto_cliente;

alter table pedidos
rename column order_date to fecha_pedido;

alter table pedidos
rename column delivery_personnel_id to personal_entrega_id;

alter table pedidos
rename column store_id to tienda_id;

alter table pedidos
rename column order_type to tipo_pedido;

alter table items_pedido
rename column order_id to pedido_id;

alter table items_pedido
rename column product_id to producto_id;

alter table items_pedido
rename column quantity to cantidad;

alter table envios
rename column order_id to pedido_id;

alter table envios
rename column shipping_status to estado_envio;

alter table envios
rename column shipping_time to tiempo_envio;

alter table entregas_programadas
rename column order_id to pedido_id;

alter table entregas_programadas
rename column scheduled_time to tiempo_programado;

alter table entregas_programadas
rename column status to estado;

alter table categorias_producto
rename column product_id to producto_id;

alter table categorias_producto
rename column category_id to categoria_id;

create table combos (
  id bigint primary key generated always as identity,
  nombre text not null,
  precio numeric(10, 2) not null
);

create table productos_combos (
  id bigint primary key generated always as identity,
  producto_id bigint references productos (id),
  combo_id bigint references combos (id),
  cantidad int not null
);

create table materiales (
  id bigint primary key generated always as identity,
  nombre text not null
);

create table materiales_asociados (
  id bigint primary key generated always as identity,
  material_id bigint references materiales (id),
  producto_id bigint references productos (id),
  material_asociado_id bigint references materiales (id)
);

alter table pedidos
add column sucursal_id bigint references sucursales (id);

alter table productos
add column sucursal_id bigint references sucursales (id);

alter table materiales_asociados
add column sucursal_id bigint references sucursales (id);

alter table tiendas
drop sucursal_id;

alter table tiendas
add column sucursal_id bigint references sucursales (id) not null;

create table sistemas_almacenamiento (
  id bigint primary key generated always as identity,
  nombre text not null
);

alter table productos
add column stock int not null default 0;

alter table productos
add column sistema_almacenamiento_id bigint references sistemas_almacenamiento (id);

alter table materiales
add column stock int not null default 0;

alter table materiales
add column sistema_almacenamiento_id bigint references sistemas_almacenamiento (id);

create table formatos_envio (
  id bigint primary key generated always as identity,
  nombre text not null
);

alter table sucursales
add column formatos_envio_aceptados bigint[0];

alter table envios
add column formato_envio_id bigint references formatos_envio (id);

alter table productos
drop constraint if exists products_store_id_fkey;

alter table pedidos
drop constraint if exists orders_store_id_fkey;

drop table if exists tiendas;

create table comercios (
  id bigint primary key generated always as identity,
  nombre text not null,
  ubicacion text not null,
  sucursal_id bigint references sucursales (id)
);

create table sucursales_formatos_envio (
  id bigint primary key generated always as identity,
  sucursal_id bigint references sucursales (id),
  formato_envio_id bigint references formatos_envio (id)
);

alter table personal_entrega
add column sucursal_id bigint references sucursales (id);

create table horarios_disponibilidad (
  id bigint primary key generated always as identity,
  personal_entrega_id bigint references personal_entrega (id),
  dia_semana text not null,
  hora_inicio time not null,
  hora_fin time not null
);

alter table horarios_disponibilidad
drop dia_semana;

alter table horarios_disponibilidad
add column dia_inicio text not null;

alter table horarios_disponibilidad
add column dia_fin text not null;

create table marcas (
  id bigint primary key generated always as identity,
  nombre text not null
);

alter table sucursales
add column marca_id bigint references marcas (id);

create table usuarios (
  id bigint primary key generated always as identity,
  nombre text not null,
  email text not null unique
);

alter table pedidos
add column usuario_id bigint references usuarios (id);

alter table marcas
add column usuario_id bigint references usuarios (id);

create table direcciones (
  id bigint primary key generated always as identity,
  latitud numeric(9, 6) not null,
  longitud numeric(9, 6) not null,
  indicaciones text not null
);

alter table direcciones
add column usuario_id bigint references usuarios (id);

alter table sucursales
add column direccion_id bigint references direcciones (id);

alter table pedidos
add column direccion_id bigint references direcciones (id);

alter table direcciones
drop usuario_id;
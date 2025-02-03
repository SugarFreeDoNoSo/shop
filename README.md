# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

Estamos desarrollando una tienda de ropa fina y elegante a medida del cliente usando Dioxus y Tailwind CSS 4. La aplicación debe seguir los principios SOLID y diseño atómico. Se necesitan los siguientes componentes:

1. **Navegador (Navbar):**
   - Diseño limpio y elegante.
   - Incluya opciones de navegación como "Inicio", "Colecciones", "Contacto" y "Carro de Compras".
   - Debe ser responsive y mantener la estética de lujo.

2. **Carro de Compras:**
   - Diseño minimalista y funcional.
   - Muestre los productos agregados, cantidades y subtotal.
   - Incluya opciones de eliminar o modificar cantidades.

3. **Base de Datos:**
   - Usar PostgreSQL como base de datos.
   - Diseñar las tablas para productos, clientes, pedidos y detalles de pedidos.
   - Asegurar la normalización y relación entre tablas.

4. **Cacheo:**
   - Implementar Redis para cachear datos frecuentemente accedidos (como listados de productos).
   - Optimizar el rendimiento de la aplicación.

5. **Pruebas:**
   - Usar Playwright para realizar pruebas end-to-end de los componentes.
   - Asegurar que el navegador y el carro de compras funcionen correctamente.
   - Pruebas de rendimiento y escalabilidad.

6. **Optimización:**
   - Maximizar el rendimiento de la aplicación.
   - Optimizar el código para reducir la complejidad y mejorar el rendimiento.
   - Implementar técnicas de lazy loading y code splitting.

**Requisitos Generales:**
- Todos los textos deben estar en español.
- Mantener una estética coherente y elegante en toda la aplicación.
- Asegurar que la aplicación sea escalable y mantenible.

**Pasos a Seguir:**
1. analiza la actual configuracion del proyecto base con Dioxus y Tailwind CSS 4.
2. Implementar los componentes siguiendo diseño atómico.
3. Configurar la base de datos con PostgreSQL.
4. Implementar Redis para el cacheo.
5. Desarrollar las pruebas con Playwright.
6. Optimizar el rendimiento de la aplicación.
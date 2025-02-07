import { test, expect } from '@playwright/test';

test.describe('Elegancia - Pruebas E2E', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8000');
  });

  test('Verificar navegación y elementos de la Navbar', async ({ page }) => {
    const navbar = page.locator('nav');
    
    // Verificar logo y enlaces
    await expect(navbar.locator('.flex-shrink-0')).toContainText('Elegancia');
    await expect(navbar).toContainText('Inicio');
    await expect(navbar).toContainText('Colecciones');
    await expect(navbar).toContainText('Contacto');
    await expect(navbar).toContainText('Carrito');

    // Probar navegación
    await page.click('text=Colecciones');
    await expect(page.locator('h2')).toContainText('Nuestras Colecciones');
    
    await page.click('text=Contacto');
    await expect(page.locator('h2')).toContainText('Contáctanos');
    
    await page.click('text=Carrito');
    await expect(page.locator('h2')).toContainText('Tu Carrito');
    
    await page.click('text=Inicio');
    await expect(page.locator('h1')).toContainText('Bienvenido a nuestra Tienda de Ropa Elegante');
  });

  test('Verificar funcionalidad del Carrito', async ({ page }) => {
    // Navegar al carrito
    await page.click('text=Carrito');
    
    // Verificar elementos del carrito
    const carritoPage = page.locator('.container');
    await expect(carritoPage.locator('h2')).toContainText('Tu Carrito');
    
    // Verificar componente del carrito
    const shoppingCart = page.locator('.max-w-2xl');
    await expect(shoppingCart).toBeVisible();
  });

  test('Verificar página de Colecciones', async ({ page }) => {
    await page.click('text=Colecciones');
    
    // Verificar título
    await expect(page.locator('h2')).toContainText('Nuestras Colecciones');
    
    // Verificar grid de productos
    const productGrid = page.locator('.grid');
    await expect(productGrid).toBeVisible();
  });

  test('Verificar página de Contacto', async ({ page }) => {
    await page.click('text=Contacto');
    
    // Verificar título
    await expect(page.locator('h2')).toContainText('Contáctanos');
    
    // Verificar formulario de contacto
    const contactForm = page.locator('.max-w-lg');
    await expect(contactForm).toBeVisible();
  });
});

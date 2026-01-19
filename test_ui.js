const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  // Set viewport
  await page.setViewportSize({ width: 1280, height: 900 });

  console.log('Loading deployed app...');
  await page.goto('https://maptoposter-production.up.railway.app/', { waitUntil: 'networkidle' });

  // Screenshot 1: Initial page load
  await page.screenshot({ path: '/tmp/ui_1_initial.png', fullPage: true });
  console.log('Screenshot 1: Initial page load saved');

  // Wait for content to load
  await page.waitForTimeout(2000);

  // Screenshot 2: After content loads
  await page.screenshot({ path: '/tmp/ui_2_loaded.png', fullPage: true });
  console.log('Screenshot 2: After content loads saved');

  // Check if theme selector exists
  const themeSelector = await page.$('select[name="theme"], #theme, .theme-select, select');
  if (themeSelector) {
    console.log('Theme selector found!');
    await themeSelector.click();
    await page.waitForTimeout(500);
    await page.screenshot({ path: '/tmp/ui_3_theme_dropdown.png', fullPage: true });
    console.log('Screenshot 3: Theme dropdown open saved');
  } else {
    console.log('Theme selector NOT found - checking page structure...');

    // Log all selects on page
    const selects = await page.$$eval('select', els => els.map(el => ({
      id: el.id,
      name: el.name,
      className: el.className,
      options: Array.from(el.options).map(o => o.value)
    })));
    console.log('Selects found:', JSON.stringify(selects, null, 2));

    // Log page title and main elements
    const title = await page.title();
    console.log('Page title:', title);

    // Check for common elements
    const bodyText = await page.textContent('body');
    console.log('Body text snippet:', bodyText.substring(0, 500));
  }

  // Screenshot the entire page HTML structure
  const html = await page.content();
  require('fs').writeFileSync('/tmp/page_html.txt', html);
  console.log('Page HTML saved to /tmp/page_html.txt');

  await browser.close();
  console.log('Done!');
})();

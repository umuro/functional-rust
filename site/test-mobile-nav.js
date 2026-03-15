#!/usr/bin/env node
/**
 * Playwright smoke test — mobile navigation visibility on /rust/
 * Usage: node site/test-mobile-nav.js [--live]
 *   --live  test https://hightechmind.io/rust/index.html
 *   (default) test local /tmp/rust-site/rust/index.html
 */

const { chromium } = require('playwright');
const path = require('path');

const LIVE = process.argv.includes('--live');
const URL = LIVE
  ? 'https://hightechmind.io/rust/index.html'
  : 'file:///tmp/rust-site/rust/index.html';

const MOBILE = { width: 390, height: 844 }; // iPhone 14

const NAV_LINKS = ['Examples', 'By Level', 'By Topic', 'Learning Paths'];

async function run() {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  await page.setViewportSize(MOBILE);

  console.log(`\n📱 Testing mobile nav at ${URL}\n`);

  await page.goto(URL, { waitUntil: 'domcontentloaded' });

  let passed = 0, failed = 0;

  for (const label of NAV_LINKS) {
    // Check in-page pills (flex-wrap pill bar in <main>)
    const locator = page.locator(`main a:has-text("${label}")`).first();
    const box = await locator.boundingBox().catch(() => null);
    const inViewport = box && box.y >= 0 && box.y < MOBILE.height && box.x < MOBILE.width;

    if (inViewport) {
      console.log(`  ✅ "${label}" — visible at (x=${Math.round(box.x)}, y=${Math.round(box.y)})`);
      passed++;
    } else {
      console.log(`  ❌ "${label}" — NOT visible on mobile`);
      failed++;
    }
  }

  // Check pill bar exists in main
  const pillBar = page.locator('main .flex.flex-wrap').first();
  const pillBarVisible = await pillBar.isVisible().catch(() => false);
  console.log(`\n  ${pillBarVisible ? '✅' : '❌'} In-page pill bar present`);
  pillBarVisible ? passed++ : failed++;

  await browser.close();

  console.log(`\nResult: ${passed} passed, ${failed} failed\n`);
  process.exit(failed > 0 ? 1 : 0);
}

run().catch(e => { console.error(e); process.exit(1); });

import { test, expect } from "@playwright/test";
import path from "path";

test.describe("Test Main Page", () => {
  test(`Check Navbar Navigation`, async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "users",
      projectName,
      "mainpage/navbar",
    );

    await page.goto("https://dev.incheonheroes.world/en/");

    await page.screenshot({
      path: `${screenshotBase}/01-mainpage.png`,
      fullPage: true,
    });
  });
});


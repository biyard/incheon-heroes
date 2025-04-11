import { test, expect } from "@playwright/test";
import path from "path";

test.describe("UI Mobile Responsiveness For Main page", () => {
  test(`Check responsiveness`, async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "users",
      "mainpage",
      projectName,
      "overflow",
    );

    await page.goto("https://dev.incheonheroes.world/en/");

    const bodyOverflowX = await page.evaluate(
      () =>
        document.documentElement.scrollWidth >
        document.documentElement.clientWidth,
    );
    expect(bodyOverflowX).toBeFalsy();

    await page.screenshot({
      path: `${screenshotBase}/01-mainpage.png`,
      fullPage: true,
    });
  });
});


import { test, expect, chromium } from "@playwright/test";
import fs from "fs";
import path from "path";

const credentials = {
    email: process.env.GOOGLE_EMAIL || "testemail@gmail.com",
    pass: process.env.GOOGLE_PASS || "thepassword",
};

const timeouts = {
    wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
    visible: parseInt(process.env.VISIBLE_TIMEOUT || "5000", 10),
    url: parseInt(process.env.URL_TIMEOUT || "7000", 10),
};

let browserInstance: any = null;

async function getBrowserInstance() {
    if (!browserInstance) {
        browserInstance = await chromium.launch({
            headless: false,
            args: [
                "--disable-blink-features=AutomationControlled",
                "--no-sandbox",
                "--disable-web-security",
                "--disable-infobars",
                "--disable-extensions",
                "--start-maximized",
                "--window-size=1300, 1280",
            ],
        });
    }
    return browserInstance;
}

test.describe.configure({ mode: "serial" });

test("Google OAuth Login and Save Session", async ({ page, context }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
        "screenshots",
        "mainpage",
        "signin",
        projectName,
        "options",
        "google",
    );

    await page.goto("https://dev.incheonheroes.world/en/");

    await page.screenshot({
        path: `${screenshotBase}/01-mainpage.png`,
        fullPage: true,
    });

    const toConnect = page.getByRole('link', { name: 'Please sign in first' })
    await expect(toConnect).toBeVisible()
    await toConnect.click()

    await page.screenshot({
        path: `${screenshotBase}/02-signin-page.png`,
        fullPage: true,
    });

    await expect(page).toHaveURL("https://dev.incheonheroes.world/en/connect")

    const googleSignIn = page.getByRole('button', { name: 'Google' })
    await expect(googleSignIn).toBeVisible()

    await page.screenshot({
        path: `${screenshotBase}/03-sigin-all-visible.png`,
        fullPage: true,
    });

    await googleSignIn.click();

    const [popup] = await Promise.all([
        page.waitForEvent("popup"),
        googleSignIn.click(),
    ]);
    await popup.screenshot({
        path: `${screenshotBase}/04-signin-page.png`,
    });

    await popup.waitForLoadState("domcontentloaded");
    await popup.waitForURL(/accounts.google.com/);

    if (await popup.isVisible('input[type="email"]')) {
        await popup.fill('input[type="email"]', credentials.email);
        await popup.click('button:has-text("Next")');
        await popup.screenshot({
            path: `${screenshotBase}/05-email-input.png`,
        });

        if (await popup.isVisible("text=Couldn’t find your Google Account")) {
            console.error("Invalid email entered!");
            return;
        }

        await popup.waitForSelector('input[type="password"]', {
            timeout: timeouts.wait,
        });
        await popup.fill('input[type="password"]', credentials.pass);
        await popup.click('button:has-text("Next")');
        await popup.screenshot({
            path: `${screenshotBase}/06-password-input.png`,
        });

        if (await popup.isVisible("text=Wrong password. Try again")) {
            console.error("Incorrect password!");
            return;
        }

        await popup.waitForLoadState("networkidle");
    }

    if (await popup.isVisible("text=This app isn’t verified")) {
        await popup.click('button:has-text("Advanced")');
        await popup.click('a:has-text("Go to https://dev.incheonheroes.world (unsafe)")');
        await popup.screenshot({
            path: `${screenshotBase}/07-verification-error.png`,
        });
    }

    await context.storageState({ path: "storage/auth.json" });

    console.log("Google OAuth login session saved!");

    await context.close();
});

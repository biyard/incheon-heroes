import { test, expect } from "@playwright/test";
import path from "path";

const credentials = {
    email: process.env.TEST_EMAIL|| "test@gmail.com",
    kakaoId: process.env.TEST_KAKAO_ID|| ""
}

test.describe("Test Main Page", () => {
    test(`Check Navbar Navigation`, async ({ page }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
            "screenshots",
            "mainpage",
            projectName,
            "signin",
        );

        await page.goto("https://dev.incheonheroes.world/en/");

        const historyAtNavbar = page.getByText('HISTORY')
        await expect(historyAtNavbar).toBeVisible()

        await page.screenshot({
            path: `${screenshotBase}/01-mainpage.png`,
            fullPage: true,
        });
    });

    test(`Navigate To Sign-In Options And Confirm Types.`, async ({ page }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
            "screenshots",
            "mainpage",
            "signin",
            projectName,
            "options",
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

        const kakaoSignIn = page.getByRole('button', { name: 'Kakao' })
        await expect(kakaoSignIn).toBeVisible()

        const kaiaSignIn = page.getByRole('button', { name: 'Kaia' })
        await expect(kaiaSignIn).toBeVisible()

        const internetSignIn = page.getByRole('button', { name: 'Internet Identity' })
        await expect(internetSignIn).toBeVisible()

        await page.screenshot({
            path: `${screenshotBase}/01-sigin-all-visible.png`,
            fullPage: true,
        });
    });


    test(`Test Kakao Sign-In Flow`, async ({ page }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
            "screenshots",
            "mainpage",
            "signin",
            projectName,
            "kakao"
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

        const kakaoSignIn = page.getByRole('button', { name: 'Kakao' })
        await expect(kakaoSignIn).toBeVisible()
        await kakaoSignIn.click()

        await expect(page).toHaveURL("https://accounts.kakao.com/login/")

        const kakaoId = page.getByRole('textbox', { name: 'Enter Account Information' })
        await expect(kakaoId).toBeVisible()
        await kakaoId.fill('testemail@gmail.com');

        await page.screenshot({
            path: `${screenshotBase}/01-kakao-signin.png`,
            fullPage: true,
        });
    });

    test(`Test Kaia Sign-In Flow`, async ({ page }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
            "screenshots",
            "mainpage",
            "signin",
            projectName,
            "kaia"
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

        const kaiaSignIn = page.getByRole('button', { name: 'Kaia' })
        await expect(kaiaSignIn).toBeVisible()
        await kaiaSignIn.click()

        await expect(page).toHaveURL("https://accounts.kaia.com/login/")

        const kaiaId = page.getByRole('textbox', { name: 'Enter Account Information' })
        await expect(kaiaId).toBeVisible()
        await kaiaId.fill('testemail@gmail.com');

        await page.screenshot({
            path: `${screenshotBase}/01-kaia-signin.png`,
            fullPage: true,
        });
    });



    test(`Test ICP Sign-In Flow`, async ({ page }, testInfo) => {
        const projectName = testInfo.project.name;
        const screenshotBase = path.join(
            "screenshots",
            "mainpage",
            "signin",
            projectName,
            "icp"
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

        const ICPSignIn = page.getByRole('button', { name: 'Kaia' })
        await expect(ICPSignIn).toBeVisible()
        await ICPSignIn.click()

        await expect(page).toHaveURL("https://identity.ic0.app/#authorize")

        const ICPStartButton = page.getByRole('button', { name: 'Use Existing' })
        await expect(ICPStartButton).toBeVisible()
        await ICPStartButton.click();

        const ICPIdentityInput = page.getByRole('textbox', { name: 'Enter Account Information' })

        await page.screenshot({
            path: `${screenshotBase}/01-icp-signin.png`,
            fullPage: true,
        });
    });


});


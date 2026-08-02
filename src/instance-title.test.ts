import { expect, it } from "vitest";
import { instanceTitle } from "./instance-title";

it("capitalises a bare instance id", () => {
  expect(instanceTitle("spectrum")).toBe("Spectrum");
});

it("splits a numbered suffix into a readable word", () => {
  expect(instanceTitle("spectrum-2")).toBe("Spectrum 2");
});

it("leaves a digit-only segment alone", () => {
  expect(instanceTitle("clock-10")).toBe("Clock 10");
});

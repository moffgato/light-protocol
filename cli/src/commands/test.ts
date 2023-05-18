import { buildPSP } from "../utils/buildPSP";
import type { Arguments, CommandBuilder, Options } from "yargs";
import { execSync } from "child_process";
import path = require("path");
import * as fs from "fs";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { toSnakeCase } from "@lightprotocol/zk.js";

export const command: string = "test";
export const desc: string = "test and deploy your PSP";

export const builder: CommandBuilder<Options> = (yargs) =>
  yargs.options({
    // network: { type: "string" },
    projectName: {type: "string"},
    programAddress: {type: "string"},
    // ptau: { type: "number" },
    // TODO: pass along anchor build options // execsync thingy alt.
  });
//TODO: move all cli-utils to cli ... -> build into bin buildPsP uses macrocircom...

export const handler = async (argv: Arguments<Options>): Promise<void> => {
  let { projectName, programAddress }: any = argv;
  const programName = toSnakeCase(projectName);
  const commandPath = path.resolve(__dirname, "../../scripts/runTest.sh");
  const systemProgramPath = path.resolve(__dirname, "../../");

  try {
    let stdout = execSync(`${commandPath} ${systemProgramPath} ${process.cwd()} ${programAddress} ${programName}.so 'yarn ts-mocha -t 2000000 tests/${projectName}.ts --exit'`)
    console.log(stdout.toString().trim());
  } catch (err) {
    console.error(err.stderr.toString());
    console.error(err.toString().trim());

  }
  process.exit(0);
};

// Function that climbs each parent directory from a given starting directory until it finds a package.json
export async function discoverFromPath(startFrom: string): Promise<string | null> {
  let currentPath: string | null = startFrom;

  while (currentPath) {
    try {
      const files = fs.readdirSync(currentPath);

      for (const file of files) {
        const filePath = path.join(currentPath, file);

        if (file === 'package.json') {
          return filePath;
        }
      }

      // Not found. Go up a directory level.
      const parentPath = path.dirname(currentPath);
      if (parentPath === currentPath) {
        currentPath = null;
      } else {
        currentPath = parentPath;
      }
    } catch (err) {
      console.error(`Error reading the directory with path: ${currentPath}`, err);
      currentPath = null;
    }
  }

  return null;
}


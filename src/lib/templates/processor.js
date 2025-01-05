const fs = require('fs/promises');
const { join, dirname } = require('path');
const inquirer = require('inquirer');

class TemplateProcessor {
    constructor(template, dest, projectName) {
        this.template = template;
        this.dest = dest;
        this.projectName = projectName;
    }

    async process() {
        // Validate template structure
        await this.validateTemplateStructure();

        // Create destination directory if it doesn't exist
        await fs.mkdir(this.dest, { recursive: true });

        // Copy and process template files
        await this.copyTemplateFiles();

        console.log('Template processed successfully!');
    }

    async validateTemplateStructure() {
        const requiredFiles = [
            'PROMPT.md',
            'CHANGELOG.md',
            'PROJECT_SCOPE.md',
            '.cursorrules'
        ];

        for (const file of requiredFiles) {
            try {
                await fs.access(join(this.template.path, file));
            } catch {
                throw new Error(`Required file ${file} is missing from template`);
            }
        }
    }

    async copyTemplateFiles() {
        await this.copyDirRecursive(this.template.path, this.dest);
    }

    async copyDirRecursive(src, dest) {
        const entries = await fs.readdir(src, { withFileTypes: true });

        for (const entry of entries) {
            const srcPath = join(src, entry.name);
            const destPath = join(dest, entry.name);

            if (entry.isDirectory()) {
                await fs.mkdir(destPath, { recursive: true });
                await this.copyDirRecursive(srcPath, destPath);
            } else {
                if (await this.fileExists(destPath)) {
                    const { overwrite } = process.env.TEST_MODE ? 
                        { overwrite: true } :
                        await inquirer.prompt([{
                            type: 'confirm',
                            name: 'overwrite',
                            message: `File '${entry.name}' already exists. Overwrite?`,
                            default: false
                        }]);

                    if (!overwrite) {
                        console.log(`Skipping file: ${entry.name}`);
                        continue;
                    }
                }
                await this.copyAndProcessFile(srcPath, destPath);
            }
        }
    }

    async copyAndProcessFile(src, dest) {
        const content = await fs.readFile(src, 'utf8');
        const processed = content.replace(/\{\{project_name\}\}/g, this.projectName);
        await fs.mkdir(dirname(dest), { recursive: true });
        await fs.writeFile(dest, processed);
    }

    async fileExists(path) {
        try {
            await fs.access(path);
            return true;
        } catch {
            return false;
        }
    }
}

module.exports = TemplateProcessor; 
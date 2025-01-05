const { expect } = require('chai');
const fs = require('fs/promises');
const path = require('path');
const os = require('os');
const TemplateProcessor = require('../src/lib/templates/processor');
const Template = require('../src/lib/templates');

describe('TemplateProcessor', () => {
    let tempDir;
    let templateDir;
    let outputDir;

    beforeEach(async () => {
        tempDir = await fs.mkdtemp(path.join(os.tmpdir(), 'create-cursor-app-'));
        templateDir = path.join(tempDir, 'template');
        outputDir = path.join(tempDir, 'output');
        
        await fs.mkdir(templateDir, { recursive: true });
        
        // Create test template files
        await fs.writeFile(
            path.join(templateDir, 'PROMPT.md'),
            '# {{project_name}}'
        );
        await fs.writeFile(
            path.join(templateDir, 'CHANGELOG.md'),
            '# Changelog for {{project_name}}'
        );
        await fs.writeFile(
            path.join(templateDir, 'PROJECT_SCOPE.md'),
            '# Scope for {{project_name}}'
        );
        await fs.writeFile(
            path.join(templateDir, '.cursorrules'),
            '{}'
        );
    });

    afterEach(async () => {
        await fs.rm(tempDir, { recursive: true, force: true });
    });

    it('should process template files', async () => {
        const template = new Template('test', 'Test Template', templateDir);
        const processor = new TemplateProcessor(template, outputDir, 'My Project');

        await processor.process();

        const content = await fs.readFile(path.join(outputDir, 'PROMPT.md'), 'utf8');
        expect(content).to.equal('# My Project');
    });

    it('should validate template structure', async () => {
        const template = new Template('test', 'Test Template', templateDir);
        const processor = new TemplateProcessor(template, outputDir, 'My Project');

        await fs.rm(path.join(templateDir, 'PROMPT.md'));

        try {
            await processor.validateTemplateStructure();
            expect.fail('Should have thrown an error');
        } catch (error) {
            expect(error.message).to.include('Required file PROMPT.md is missing');
        }
    });

    it('should handle file overwrite confirmation', async () => {
        const template = new Template('test', 'Test Template', templateDir);
        const processor = new TemplateProcessor(template, outputDir, 'My Project');

        // Create existing file
        await fs.mkdir(outputDir, { recursive: true });
        await fs.writeFile(
            path.join(outputDir, 'PROMPT.md'),
            'existing content'
        );

        // Set test mode to auto-confirm overwrites
        process.env.TEST_MODE = '1';
        await processor.process();

        const content = await fs.readFile(path.join(outputDir, 'PROMPT.md'), 'utf8');
        expect(content).to.equal('# My Project');
    });
}); 
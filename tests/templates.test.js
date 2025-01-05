const { expect } = require('chai');
const { join } = require('path');
const fs = require('fs/promises');
const Template = require('../src/lib/templates');
const os = require('os');
const path = require('path');

describe('Template', () => {
    let tempDir;

    beforeEach(async () => {
        // Create a temporary directory for tests
        tempDir = await fs.mkdtemp(join(os.tmpdir(), 'create-cursor-app-'));
        
        // Create test templates directory
        const templatesDir = join(tempDir, 'templates');
        await fs.mkdir(templatesDir);
        
        // Create a basic template
        const basicTemplateDir = join(templatesDir, 'basic');
        await fs.mkdir(basicTemplateDir);
        
        // Create required files
        await fs.writeFile(
            join(basicTemplateDir, 'PROMPT.md'),
            '# {{project_name}}'
        );
        await fs.writeFile(
            join(basicTemplateDir, 'CHANGELOG.md'),
            '# Changelog for {{project_name}}'
        );
        await fs.writeFile(
            join(basicTemplateDir, 'PROJECT_SCOPE.md'),
            '# Scope for {{project_name}}'
        );
        await fs.writeFile(
            join(basicTemplateDir, '.cursorrules'),
            JSON.stringify({
                settings: {
                    context_tracking: {
                        enabled: true,
                        files: ["PROMPT.md", "CHANGELOG.md", "PROJECT_SCOPE.md"]
                    }
                }
            })
        );
    });

    afterEach(async () => {
        // Clean up temporary directory
        await fs.rm(tempDir, { recursive: true, force: true });
    });

    describe('Template.loadFromDir', () => {
        it('should load templates from directory', async () => {
            const templates = await Template.loadFromDir(join(tempDir, 'templates'));
            expect(templates).to.have.lengthOf(1);
            expect(templates[0].name).to.equal('basic');
        });
    });

    describe('Template constructor', () => {
        it('should create template with default config', () => {
            const template = new Template('test', 'Test Template', '/path/to/template');
            expect(template.name).to.equal('test');
            expect(template.description).to.equal('Test Template');
            expect(template.config).to.deep.include({
                dependencies: [],
                cursorrules: {
                    language: '',
                    framework: null,
                    settings: null
                }
            });
        });
    });
}); 
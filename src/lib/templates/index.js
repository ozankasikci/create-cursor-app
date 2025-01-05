const { join } = require('path');
const fs = require('fs/promises');

class Template {
    constructor(name, description, path) {
        this.name = name;
        this.description = description;
        this.path = path;
        this.config = Template.defaultConfig();
    }

    static defaultConfig() {
        return {
            dependencies: [],
            cursorrules: {
                language: '',
                framework: null,
                settings: null
            },
            post_install: []
        };
    }

    static async loadAll() {
        return Template.loadFromDir(join(process.cwd(), 'templates'));
    }

    static async loadFromDir(templatesDir) {
        const templates = [];
        try {
            const entries = await fs.readdir(templatesDir, { withFileTypes: true });
            
            for (const entry of entries) {
                if (entry.isDirectory()) {
                    templates.push(new Template(
                        entry.name,
                        'Template description',
                        join(templatesDir, entry.name)
                    ));
                }
            }
        } catch (error) {
            console.error('Error loading templates:', error);
        }
        return templates;
    }
}

module.exports = Template; 
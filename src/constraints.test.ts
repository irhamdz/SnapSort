import { describe, it, expect } from 'vitest'
import { existsSync, readFileSync } from 'fs'
import { globSync } from 'glob'

describe('Constraint Compliance Tests', () => {
  it('should not have any CSS module files (.module.css)', () => {
    const cssModuleFiles = globSync('**/*.module.css')
    expect(cssModuleFiles).toEqual([])
  })

  it('should not have inline styles (style={{}}) in TypeScript files', () => {
    const tsFiles = globSync('src/**/*.ts')
    const tsxFiles = globSync('src/**/*.tsx')

    const files = [...tsFiles, ...tsxFiles]

    files.forEach((file) => {
      if (file.includes('test')) return

      const content = readFileSync(file, 'utf-8')
      const hasInlineStyles = content.includes('style={{') || content.includes('style={"')
      expect(hasInlineStyles).toBe(false)
    })
  })

  it('should not have Redux imports in stores', () => {
    const storeFiles = globSync('src/stores/**/*.ts')

    storeFiles.forEach((file) => {
      const content = readFileSync(file, 'utf-8')
      const hasRedux = content.includes('redux') || content.includes('createStore') || content.includes('combineReducers')
      expect(hasRedux).toBe(false)
    })
  })

  it('should not have MobX imports in stores', () => {
    const storeFiles = globSync('src/stores/**/*.ts')

    storeFiles.forEach((file) => {
      const content = readFileSync(file, 'utf-8')
      const hasMobX = content.includes('mobx') || content.includes('makeAutoObservable') || content.includes('observable')
      expect(hasMobX).toBe(false)
    })
  })

  it('should not have React Context imports in stores', () => {
    const storeFiles = globSync('src/stores/**/*.ts')

    storeFiles.forEach((file) => {
      const content = readFileSync(file, 'utf-8')
      const hasContext = content.includes('createContext') || content.includes('useContext')
      // Context is allowed in components, just not in stores
      if (!file.includes('test')) {
        expect(hasContext).toBe(false)
      }
    })
  })

  it('should not have direct invoke() calls in components or stores', () => {
    const componentFiles = globSync('src/components/**/*.tsx')
    const storeFiles = globSync('src/stores/**/*.ts')
    const testFiles = globSync('src/**/*test*.ts')

    const files = [...componentFiles, ...storeFiles, ...testFiles]

    files.forEach((file) => {
      const content = readFileSync(file, 'utf-8')
      const hasDirectInvoke = content.includes("invoke(") && !content.includes('src/api')
      expect(hasDirectInvoke).toBe(false)
    })
  })

  it('should not have inline SQL string in Rust commands', () => {
    const commandFiles = globSync('src-tauri/src/commands/**/*.rs')

    commandFiles.forEach((file) => {
      const content = readFileSync(file, 'utf-8')
      // Check for raw SQL strings (single or double quotes)
      const hasRawSQL = /['"`]INSERT|['"`]UPDATE|['"`]DELETE|['"`]SELECT|['"`]CREATE|['"`]ALTER|['"`]DROP|['"`]VALUES|['"`]FROM|['"`]WHERE|['"`]JOIN|['"`]ORDER|['"`]GROUP|['"`]HAVING|['"`]LIMIT|['"`]OFFSET|['"`]INTO|['"`]AS|['"`]DISTINCT|['"`]ALL|['"`]EXISTS|['"`]IN|['"`]BETWEEN|['"`]LIKE|['"`]IS|['"`]NULL|['"`]AND|['"`]OR|['"`]NOT|['"`]CASE|['"`]WHEN|['"`]THEN|['"`]ELSE|['"`]END|['"`]ON|['"`]SET|['"`]TABLE|['"`]INDEX|['"`]PRIMARY|['"`]FOREIGN|['"`]KEY|['"`]REFERENCES|['"`]UNIQUE|['"`]CHECK|['"`]DEFAULT|['"`]CONSTRAINT|['"`]REFERENCES|['"`]ON|['"`]CASCADE|['"`]RESTRICT|['"`]NO|['"`]ACTION|['"`]DEFERRABLE|['"`]INITIALLY|['"`]DEFERRED|['"`]IMMEDIATE|['"`]WITH|['"`]RECURSIVE|['"`]RECURSIVELY|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE|['"`]RECURSIVE/g.test(content)
      expect(hasRawSQL).toBe(false)
    })
  })

  it('should have all three Zustand store slices', () => {
    const storeFiles = globSync('src/stores/**/*.ts')
    const testFiles = globSync('src/stores/**/*test*.ts')

    const nonTestStoreFiles = storeFiles.filter((f) => !testFiles.includes(f))

    nonTestStoreFiles.forEach((file) => {
      const content = readFileSync(file, 'utf-8')
      expect(content).toContain('export const use')
      expect(content).toContain('Store')
    })

    const galleryStore = globSync('src/stores/useGalleryStore.ts')
    const batchStore = globSync('src/stores/useBatchStore.ts')
    const settingsStore = globSync('src/stores/useSettingsStore.ts')

    expect(galleryStore.length).toBe(1)
    expect(batchStore.length).toBe(1)
    expect(settingsStore.length).toBe(1)
  })

  it('should have src/api/index.ts as the sole invoke wrapper', () => {
    const apiFiles = globSync('src/api/**/*.ts')
    const apiTestFiles = globSync('src/api/**/*test*.ts')

    const apiSourceFiles = apiFiles.filter((f) => !apiTestFiles.includes(f))

    expect(apiSourceFiles).toHaveLength(1)
    expect(apiSourceFiles[0]).toContain('api/index.ts')
  })

  it('should have main.tsx entry point', () => {
    expect(existsSync('src/main.tsx')).toBe(true)
  })

  it('should have App.tsx component', () => {
    expect(existsSync('src/App.tsx')).toBe(true)
  })

  it('should have styles.css Tailwind entry', () => {
    expect(existsSync('src/styles.css')).toBe(true)
  })

  it('should have vite-env.d.ts type declarations', () => {
    expect(existsSync('src/vite-env.d.ts')).toBe(true)
  })
})
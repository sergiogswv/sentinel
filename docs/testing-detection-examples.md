# Testing Framework Detection - Examples

## Overview

Sentinel v4.5.0 introduces intelligent testing framework detection that analyzes your project and provides contextual recommendations based on your main framework.

## Example Outputs

### Example 1: NestJS Project with Jest (Valid Configuration)

```bash
🧪 Detectando frameworks de testing...
   ✅ Análisis de testing completado

═══ ANÁLISIS DE TESTING ═══
   ✅ Testing configurado correctamente
   📦 Framework principal: Jest
   🔧 Frameworks adicionales: Supertest
   📄 Configuración encontrada:
      • jest.config.js
      • package.json

═══════════════════════════
```

### Example 2: React Project without Testing (Missing)

```bash
🧪 Detectando frameworks de testing...
   ✅ Análisis de testing completado

═══ ANÁLISIS DE TESTING ═══
   ❌ No se detectaron frameworks de testing
   💡 Se recomienda configurar testing para el proyecto

   SUGERENCIAS DE INSTALACIÓN:

   🔥 1. Jest
      📝 El estándar para testing en React con excelente soporte
      💻 npm install --save-dev jest @types/jest

   ⭐ 2. Vitest
      📝 Alternativa moderna y rápida, compatible con Vite
      💻 npm install --save-dev vitest

   💡 3. Cypress
      📝 Para testing E2E de componentes React
      💻 npm install --save-dev cypress

═══════════════════════════
```

### Example 3: Django Project with Incomplete Config

```bash
🧪 Detectando frameworks de testing...
   ✅ Análisis de testing completado

═══ ANÁLISIS DE TESTING ═══
   ⚠️ Configuración de testing incompleta
   📦 Framework detectado: Pytest
   💡 Recomendación: Completar configuración o instalar herramientas

   SUGERENCIAS DE INSTALACIÓN:

   🔥 1. Pytest
      📝 El estándar moderno para testing en Python
      💻 pip install pytest pytest-cov

   ⭐ 2. Coverage.py
      📝 Para análisis de cobertura
      💻 pip install coverage

═══════════════════════════
```

### Example 4: Laravel Project (Valid)

```bash
🧪 Detectando frameworks de testing...
   ✅ Análisis de testing completado

═══ ANÁLISIS DE TESTING ═══
   ✅ Testing configurado correctamente
   📦 Framework principal: PHPUnit
   📄 Configuración encontrada:
      • phpunit.xml
      • composer.json

═══════════════════════════
```

## Configuration Output

The detection adds these fields to `.sentinelrc.toml`:

```toml
[config]
version = "4.5.0"
project_name = "my-project"
framework = "NestJS"
# ... other config ...

# Testing framework detection
testing_framework = "Jest"
testing_status = "valid"  # or "incomplete" or "missing"
```

## Supported Frameworks by Technology

### JavaScript/TypeScript
- **Jest**: Default for React, NestJS, Node.js
- **Vitest**: Modern alternative, great for Vite projects
- **Cypress**: E2E testing
- **Playwright**: Modern E2E testing
- **Mocha**: Flexible testing framework
- **Jasmine**: Default for Angular

### Python
- **Pytest**: Industry standard
- **Unittest**: Built-in testing
- **Coverage.py**: Code coverage analysis

### PHP
- **PHPUnit**: Standard for PHP
- **Pest**: Modern, elegant alternative
- **Laravel Dusk**: Browser testing for Laravel

### Rust
- **Built-in**: Native Rust testing with `#[cfg(test)]`
- **cargo-tarpaulin**: Code coverage tool

### Go
- **Go Testing**: Native `testing` package
- **testify**: Popular assertion library
- **httptest**: HTTP testing utilities

### Java/Spring
- **JUnit 5**: Modern testing framework
- **Spring Test**: Spring-specific testing
- **Mockito**: Mocking framework

## AI-Enhanced Recommendations

The testing detection uses AI to:

1. **Validate detected frameworks**: Ensures accuracy
2. **Contextualize suggestions**: Provides framework-specific recommendations
3. **Prioritize options**: Ranks suggestions by relevance
4. **Generate commands**: Creates installation commands for your package manager

## Integration with `sentinel init`

Testing detection runs automatically during project initialization:

```bash
sentinel init

# ... framework detection ...

🧪 Detectando frameworks de testing...

# ... testing analysis and recommendations ...

✅ Configuración actualizada.
```

## Command Generation

The system automatically generates the correct installation command based on:
- **Framework**: What you're installing (Jest, Pytest, etc.)
- **Package manager**: npm, yarn, pnpm, pip, composer
- **Project framework**: Additional context for dependencies

### Examples:

#### npm project
```bash
npm install --save-dev jest @types/jest
```

#### yarn project
```bash
yarn add --dev vitest
```

#### Python project
```bash
pip install pytest pytest-cov
```

#### PHP project
```bash
composer require --dev phpunit/phpunit
```

## Benefits

1. **Time Saving**: No need to research which testing framework to use
2. **Best Practices**: Recommendations follow industry standards
3. **Context-Aware**: Suggestions match your specific framework
4. **Easy Setup**: Copy-paste installation commands
5. **Validation**: Confirms existing configurations are complete

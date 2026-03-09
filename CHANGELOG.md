# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Advanced Tool Calling Patterns**: Integrated advanced tool calling approaches to reduce token usage and improve agent efficiency, particularly when working with a large number of tools or data points.
  - **Tool Search (Búsqueda de herramientas)**: Solves the context window issue by dynamically loading tools only when necessary, minimizing token consumption caused by static tool definitions.
  - **Programmatic Tool Calling (Llamada programática a herramientas)**: Enables LLMs to write and execute code to orchestrate tool calls within a loop (e.g., iterating through multiple data points like budget analysis), rather than making repetitive individual calls. This improves accuracy and token efficiency.
  - **Tool Bridge Architecture (Arquitectura de puente de herramientas)**: Securely routes tool calls from the code execution sandbox through the backend application without direct internet access or API credentials.
  - **Tool Usage Examples (Ejemplos de uso de herramientas)**: Added extensive tool usage examples which drastically improve accuracy in handling complex parameters (improving precision from 72% to 90%).

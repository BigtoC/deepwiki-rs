# Litho 系统架构文档

## 1. 系统上下文图

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph ExternalSystems
        LLMClient --> LLMService
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    style Litho fill:#f9f,stroke:#333;
    style ExternalSystems fill:#f9f,stroke:#333;
```

## 2. 容器图

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 3. 组件图

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 4. 代码图

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 5. 系统上下文图（详细版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph ExternalSystems
        LLMClient --> LLMService
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph LLMService
        LLMService --> API
        LLMService --> Model
    end

    subgraph Markdown
        Markdown --> Parser
        Markdown --> Renderer
    end

    subgraph HTML
        HTML --> Parser
        HTML --> Renderer
    end

    subgraph PDF
        PDF --> Parser
        PDF --> Renderer
    end

    style Litho fill:#f9f,stroke:#333;
    style ExternalSystems fill:#f9f,stroke:#333;
    style LLMService fill:#f9f,stroke:#333;
    style Markdown fill:#f9f,stroke:#333;
    style HTML fill:#f9f,stroke:#333;
    style PDF fill:#f9f,stroke:#333;
```

## 6. 容器图（详细版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    subgraph LLMService
        LLMService --> API
        LLMService --> Model
    end

    subgraph Markdown
        Markdown --> Parser
        Markdown --> Renderer
    end

    subgraph HTML
        HTML --> Parser
        HTML --> Renderer
    end

    subgraph PDF
        PDF --> Parser
        PDF --> Renderer
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
    style LLMService fill:#f9f,stroke:#333;
    style Markdown fill:#f9f,stroke:#333;
    style HTML fill:#f9f,stroke:#333;
    style PDF fill:#f9f,stroke:#333;
```

## 7. 组件图（详细版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    subgraph LLMService
        LLMService --> API
        LLMService --> Model
    end

    subgraph Markdown
        Markdown --> Parser
        Markdown --> Renderer
    end

    subgraph HTML
        HTML --> Parser
        HTML --> Renderer
    end

    subgraph PDF
        PDF --> Parser
        PDF --> Renderer
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
    style LLMService fill:#f9f,stroke:#333;
    style Markdown fill:#f9f,stroke:#333;
    style HTML fill:#f9f,stroke:#333;
    style PDF fill:#f9f,stroke:#333;
```

## 8. 代码图（详细版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    subgraph LLMService
        LLMService --> API
        LLMService --> Model
    end

    subgraph Markdown
        Markdown --> Parser
        Markdown --> Renderer
    end

    subgraph HTML
        HTML --> Parser
        HTML --> Renderer
    end

    subgraph PDF
        PDF --> Parser
        PDF --> Renderer
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
    style LLMService fill:#f9f,stroke:#333;
    style Markdown fill:#f9f,stroke:#333;
    style HTML fill:#f9f,stroke:#333;
    style PDF fill:#f9f,stroke:#333;
```

## 9. 系统上下文图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph ExternalSystems
        LLMClient --> LLMService
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    style Litho fill:#f9f,stroke:#333;
    style ExternalSystems fill:#f9f,stroke:#333;
```

## 10. 容器图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 11. 组件图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 12. 代码图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 13. 系统上下文图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph ExternalSystems
        LLMClient --> LLMService
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    style Litho fill:#f9f,stroke:#333;
    style ExternalSystems fill:#f9f,stroke:#333;
```

## 14. 容器图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 15. 组件图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 16. 代码图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 17. 系统上下文图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph ExternalSystems
        LLMClient --> LLMService
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    style Litho fill:#f9f,stroke:#333;
    style ExternalSystems fill:#f9f,stroke:#333;
```

## 18. 容器图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 19. 组件图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```

## 20. 代码图（简化版本）

```mermaid
graph TD
    subgraph Litho
        LithoCore --> LLMClient
        LithoCore --> MetadataExtractor
        LithoCore --> DocumentGenerator
        LithoCore --> ReactAgent
        LithoCore --> Tools
        LithoCore --> Utils
    end

    subgraph LLMClient
        LLMClient --> LLMService
    end

    subgraph MetadataExtractor
        MetadataExtractor --> CodeAnalyzer
        MetadataExtractor --> ArchitectureDetector
        MetadataExtractor --> FileExplorer
    end

    subgraph DocumentGenerator
        DocumentGenerator --> Markdown
        DocumentGenerator --> HTML
        DocumentGenerator --> PDF
    end

    subgraph ReactAgent
        ReactAgent --> LLMClient
        ReactAgent --> MetadataExtractor
        ReactAgent --> Tools
    end

    subgraph Tools
        Tools --> LLMClient
        Tools --> Utils
    end

    subgraph Utils
        Utils --> FileSystem
        Utils --> StringProcessing
        Utils --> DataStructures
        Utils --> PathHandling
        Utils --> TimeHandling
        Utils --> ErrorHandling
    end

    style Litho fill:#f9f,stroke:#333;
    style LLMClient fill:#f9f,stroke:#333;
    style MetadataExtractor fill:#f9f,stroke:#333;
    style DocumentGenerator fill:#f9f,stroke:#333;
    style ReactAgent fill:#f9f,stroke:#333;
    style Tools fill:#f9f,stroke:#333;
    style Utils fill:#f9f,stroke:#333;
```
# Cortex Business & Licensing Strategy
## Plan Estratégico para Monetización y Distribución

---

## 🎯 Visión

**Cortex** será la plataforma de memoria cognitiva líder para agentes de IA, con:
- ✅ Open Source (licencia BSL)
- ✅ Cloud-agnostic (AWS, GCP, Azure, Cloudflare)
- ✅ Capa gratuita generosa
- ✅ Enterprise features de pago

---

## 📜 Modelo de Licencia: Business Source License (BSL)

Inspirado en **SurrealDB**, usaremos **BSL 1.1**:

### Lo que PERMITE la licencia:
- ✅ Uso personal y comercial
- ✅ Ejecutar nodos propios
- ✅ Crear aplicaciones sobre Cortex
- ✅ Modificar para uso interno
- ✅ Instalar en cualquier cloud

### Lo que RESTRINGE:
- ❌ **No puedes vender Cortex como producto standalone**
- ❌ **No puedes ofrecer Cortex como servicio SaaS** sin autorización
- ❌ **No puedes crear forks comerciales** directo

### Excepciones (Licencias especiales):
- **SouthWest AI Labs** (nosotros): ✅ Full rights
- **Tripro.cl**: ✅ Full rights (socio estratégico)
- **Partners certificados**: ✅ Licencia comercial disponible

### Cambio después de 3 años:
- BSL cambia a **Apache 2.0** (MIT compatible)
- Cualquiera puede usar sin restricciones

---

## 💰 Modelo de Ingresos

### 1. Cortex Cloud (SaaS Propio)
| Plan | Precio | Features |
|------|--------|----------|
| **Free** | $0 | 1K memories, 100 queries/day |
| **Starter** | $29/mo | 10K memories, 10K queries/day |
| **Pro** | $99/mo | 100K memories, 100K queries/day |
| **Enterprise** | Custom | Unlimited + SLA |

### 2. Licencias Enterprise
| Tipo | Precio | Incluye |
|------|--------|---------|
| **On-premise** | $5K-50K/yr | Soporte, training, updates |
| **Cloud deployment** | $2K-20K/yr | Deployment en tu cloud |
| **Multi-node** | Custom | Clusters, replication |

### 3. Servicios Profesional
- **Implementation**: $5K-50K
- **Training**: $1K-5K
- **Custom development**: $150-300/hr

### 4. Nodos como Servicio (Cortex Network)
- Usuarios pueden crear nodos
- Ganan credits por compartir recursos
- Nosotros tomamos % del uso

---

## ☁️ Estrategia Multi-Cloud

### Objetivo
Ser el **standard de memoria para agentes IA** en cualquier cloud.

### Proveedores Soportados

| Cloud | Tipo Deployment | Status |
|-------|-----------------|--------|
| **AWS** | ECS, Lambda, EKS | ✅ Plan |
| **GCP** | Cloud Run, GKE | ✅ Plan |
| **Azure** | Container Apps, AKS | ✅ Plan |
| **Cloudflare** | Workers + Durable Objects | ✅ Plan |
| **Self-hosted** | Docker, Kubernetes | ✅ Listo |

### Diferenciador vs Byrover

| Feature | Byrover | Cortex |
|---------|---------|--------|
| Multi-cloud | ❌ Solo | ✅ Todos |
| Free tier | ? | ✅ Generosa |
| Self-hosted | ❌ | ✅ |
| Belief Graph | ❌ | ✅ Unique |
| Open Source | ❌ | ✅ BSL |
| Price | Alto | Accesible |

---

## 📊 Comparativa de Mercado

### Competidores

| Producto | Tipo | Fortalezas | Debilidades |
|----------|------|------------|--------------|
| **Byrover** | SaaS | Benchmark good | Caro, solo cloud |
| **Mem0** | Open Source | Popular | Basic memory |
| **LangChain Memory** | Library | Integrado | Limitado |
| **SurrealDB** | Database | Full DB | No es memory-first |
| **Cortex** | Memory-first | Unique features | Nuevo |

### Nuestra Ventaja Competitiva
1. **Belief Graph** - Representación única de conocimiento
2. **Multi-cloud** - Donde el cliente quiera
3. **BSL** - Transpareente y justo
4. **Precio** - Mucho más accesible
5. **Open Source** - Comunidad y confianza

---

## 🏢 Estructura Corporativa

### Authorized Entities (Tienen full rights)

```
SouthWest AI Labs (SWAL)
├── Dueño del proyecto
├── Licencias comerciales
└── Revenue share

Tripro.cl (Socio estratégico)
├── uso full de Cortex
├── integración con ManteniApp
└── revenue share en ventas conjuntas
```

### Partners Program

| Nivel | Requisitos | Beneficios |
|-------|------------|------------|
| **Silver** | 1K/mo revenue | Discount licenses |
| **Gold** | 10K/mo revenue | Priority support |
| **Platinum** | 50K/mo revenue | Custom terms |

---

## 🚀 Roadmap de Monetización

### Fase 1: Launch (Mes 1-2)
- [ ] Release open source (BSL)
- [ ] Documentación completa
- [ ] Docker images
- [ ] Free tier (limitar uso)

### Fase 2: Cloud Beta (Mes 3-4)
- [ ] Cortex Cloud en beta
- [ ] Paid tiers (Starter, Pro)
- [ ] Dashboard de payments

### Fase 3: Enterprise (Mes 5-6)
- [ ] On-premise licenses
- [ ] Multi-cloud deployments
- [ ] Partner program

### Fase 4: Network (Mes 7-12)
- [ ] Cortex Network (nodos)
- [ ] Credits system
- [ ] Marketplace

---

## 📋 Legal Text (Licencia)

```text
CORTEX BUSINESS SOURCE LICENSE 1.1
Copyright (c) 2026 SouthWest AI Labs

PARAMETERS
Licensor: SouthWest AI Labs
Licensed Work: Cortex 0.2.0

USO PERMITIDO:
- Uso personal y comercial
- Ejecutar nodos propios
- Crear aplicaciones
- Modificar para uso interno

RESTRICCIONES:
- No vender Cortex standalone
- No ofrecer como SaaS sin licencia
- No crear forks comerciales

EXCEPCIONES:
- SouthWest AI Labs: Full rights
- Tripro.cl: Full rights (socio)
- Partners certificados: Licencia disponible

DESPUÉS DE 3 AÑOS:
Cambia a Apache 2.0 (uso libre)
```

---

## 📈 Proyección de Ingresos

| Año | ARR Target | Clientes |
|-----|-----------|---------|
| 2026 | $50K | 20 |
| 2027 | $200K | 100 |
| 2028 | $500K | 300 |
| 2029 | $1M+ | 1000+ |

---

## ✅ Action Items Inmediatos

1. **Cambiar LICENSE a BSL**
2. **Crear LICENSE.es.md** (versión español)
3. **Setup Stripe** para payments
4. **Landing page** con pricing
5. **Docs para cloud deployment**
6. **Partner agreement template**

---

## 🔑 Keys del Éxito

1. **Ser el estándar** de memoria para agentes
2. **Multi-cloud** = flexibility = trust
3. **Precio justo** vs competidores
4. **Comunidad open source** = growth
5. **Enterprise** = revenue

---

*Documento creado: 2026-03-11*
*Versión: 1.0*

# Clutch Protocol Monitoring

This directory contains monitoring configuration for the Clutch Protocol blockchain network.

**Location**: `config/monitoring/` - All monitoring configs are organized with other configuration files.

## Components

### Prometheus (`prometheus/`)
- **prometheus.yml**: Main Prometheus configuration
- Scrapes metrics from all Clutch nodes (ports 3001-3003)
- Retention: 200 hours
- Web UI: http://localhost:9090

### Grafana (`grafana/`)
- **datasources.yml**: Auto-configures Prometheus as data source
- **dashboards.yml**: Dashboard provisioning configuration
- Web UI: http://localhost:3000 (admin/admin)

## Usage

```bash
# Start all monitoring services
docker-compose up -d prometheus grafana

# View Prometheus targets
curl http://localhost:9090/api/v1/targets

# Access Grafana
open http://localhost:3000
```

## Metrics Endpoints

- **Node 1**: http://localhost:3001/metrics
- **Node 2**: http://localhost:3002/metrics  
- **Node 3**: http://localhost:3003/metrics
- **Prometheus**: http://localhost:9090/metrics

## Configuration

All configuration files are version controlled and should be modified in this repository, not externally.

### Adding New Targets

Edit `config/monitoring/prometheus/prometheus.yml`:

```yaml
scrape_configs:
  - job_name: "new-service"
    static_configs:
      - targets: ["service:port"]
```

### Custom Dashboards

Place JSON dashboard files in `config/monitoring/grafana/dashboards/` and they'll be auto-imported.

## Best Practices

✅ **DO**:
- Keep all configs in source control
- Use relative paths in docker-compose
- Version control dashboard JSON files
- Document metric endpoints

❌ **DON'T**:
- Store configs outside the repository
- Use absolute host paths for configs
- Hardcode credentials in configs

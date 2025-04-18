-- Initial database schema setup

-- Create meta schema for platform metadata
CREATE SCHEMA IF NOT EXISTS meta;

-- Tenant management
CREATE TABLE meta.tenants (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) NOT NULL UNIQUE,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    config JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- User management
CREATE TABLE meta.users (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    email VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE,
    UNIQUE(tenant_id, email)
);

-- Role definition
CREATE TABLE meta.roles (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name)
);

-- Role permissions
CREATE TABLE meta.permissions (
    id UUID PRIMARY KEY,
    role_id UUID REFERENCES meta.roles(id),
    resource VARCHAR(100) NOT NULL,
    action VARCHAR(100) NOT NULL,
    constraints JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(role_id, resource, action)
);

-- User role assignments
CREATE TABLE meta.user_roles (
    user_id UUID REFERENCES meta.users(id),
    role_id UUID REFERENCES meta.roles(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Product definitions
CREATE TABLE meta.products (
    id VARCHAR(50) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Subscription plans
CREATE TABLE meta.plans (
    id UUID PRIMARY KEY,
    product_id VARCHAR(50) REFERENCES meta.products(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    duration_type VARCHAR(50) NOT NULL,
    duration_value INTEGER NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'IDR',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    features JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Tenant subscriptions
CREATE TABLE meta.subscriptions (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    plan_id UUID REFERENCES meta.plans(id),
    status VARCHAR(50) NOT NULL,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    trial_end_date TIMESTAMP WITH TIME ZONE,
    auto_renew BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    canceled_at TIMESTAMP WITH TIME ZONE
);

-- Entity definitions (for no-code schema builder)
CREATE TABLE meta.entities (
    id VARCHAR(100) NOT NULL,
    tenant_id UUID REFERENCES meta.tenants(id),
    product_id VARCHAR(50) REFERENCES meta.products(id),
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    schema JSONB NOT NULL,
    ui_schema JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, id)
);

-- UI definitions
CREATE TABLE meta.ui_definitions (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    entity_id VARCHAR(100) NOT NULL,
    view_type VARCHAR(50) NOT NULL,
    definition JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (tenant_id, entity_id) REFERENCES meta.entities(tenant_id, id),
    UNIQUE(tenant_id, entity_id, view_type)
);

-- Workflow definitions
CREATE TABLE meta.workflows (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    entity_id VARCHAR(100),
    trigger_type VARCHAR(50) NOT NULL,
    trigger_config JSONB NOT NULL,
    steps JSONB NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    FOREIGN KEY (tenant_id, entity_id) REFERENCES meta.entities(tenant_id, id)
);

-- File storage metadata
CREATE TABLE meta.files (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    name VARCHAR(255) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    size BIGINT NOT NULL,
    storage_path VARCHAR(512) NOT NULL,
    access_type VARCHAR(50) NOT NULL DEFAULT 'private',
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Notification templates
CREATE TABLE meta.notification_templates (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    subject VARCHAR(255),
    content TEXT NOT NULL,
    variables JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, name, type)
);

-- Notifications
CREATE TABLE meta.notifications (
    id UUID PRIMARY KEY,
    tenant_id UUID REFERENCES meta.tenants(id),
    user_id UUID REFERENCES meta.users(id),
    template_id UUID REFERENCES meta.notification_templates(id),
    type VARCHAR(50) NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    read BOOLEAN NOT NULL DEFAULT FALSE,
    data JSONB,
    sent_at TIMESTAMP WITH TIME ZONE,
    read_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Insert default products
INSERT INTO meta.products (id, name, description, status)
VALUES
    ('klolatoko', 'Klolatoko', 'Retail management system', 'active'),
    ('klolakos', 'Klolakos', 'Property management system', 'active'),
    ('klolarental', 'Klolarental', 'Rental management system', 'active'),
    ('klolaform', 'Klolaform', 'Form builder system', 'active');

-- Insert default plans
INSERT INTO meta.plans (id, product_id, name, description, duration_type, duration_value, price, currency, features)
VALUES
    (gen_random_uuid(), 'klolatoko', 'Basic', 'Basic retail management plan', 'month', 1, 99000, 'IDR', '{"max_products": 100, "max_users": 3}'::jsonb),
    (gen_random_uuid(), 'klolatoko', 'Pro', 'Professional retail management plan', 'month', 1, 299000, 'IDR', '{"max_products": 1000, "max_users": 10}'::jsonb),
    (gen_random_uuid(), 'klolakos', 'Basic', 'Basic property management plan', 'month', 1, 199000, 'IDR', '{"max_properties": 5, "max_users": 3}'::jsonb),
    (gen_random_uuid(), 'klolakos', 'Pro', 'Professional property management plan', 'month', 1, 499000, 'IDR', '{"max_properties": 20, "max_users": 10}'::jsonb),
    (gen_random_uuid(), 'klolarental', 'Basic', 'Basic rental management plan', 'month', 1, 149000, 'IDR', '{"max_vehicles": 10, "max_users": 3}'::jsonb),
    (gen_random_uuid(), 'klolarental', 'Pro', 'Professional rental management plan', 'month', 1, 399000, 'IDR', '{"max_vehicles": 50, "max_users": 10}'::jsonb),
    (gen_random_uuid(), 'klolaform', 'Basic', 'Basic form builder plan', 'month', 1, 49000, 'IDR', '{"max_forms": 5, "max_responses": 100}'::jsonb),
    (gen_random_uuid(), 'klolaform', 'Pro', 'Professional form builder plan', 'month', 1, 149000, 'IDR', '{"max_forms": 20, "max_responses": 1000}'::jsonb);

-- Function to create a new tenant schema
CREATE OR REPLACE FUNCTION create_tenant_schema(tenant_slug VARCHAR) RETURNS VOID AS $$
BEGIN
    EXECUTE 'CREATE SCHEMA IF NOT EXISTS tenant_' || tenant_slug;
 END;
$$ LANGUAGE plpgsql;

-- Function to set search path for a tenant
CREATE OR REPLACE FUNCTION set_tenant_search_path(tenant_slug VARCHAR) RETURNS VOID AS $$
BEGIN
    EXECUTE 'SET search_path TO tenant_' || tenant_slug || ', public';
END;
$$ LANGUAGE plpgsql;

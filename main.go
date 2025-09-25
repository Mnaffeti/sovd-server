package main

import (
	"log"
	"sovd-server/internal/handlers"
	"sovd-server/internal/services"

	"github.com/gin-gonic/gin"
)

// @title SOVD Server API
// @version 1.0
// @description Service Oriented Vehicle Diagnostics (SOVD) Server API implementation
// @termsOfService http://swagger.io/terms/

// @contact.name API Support
// @contact.url http://www.example.com/support
// @contact.email support@example.com

// @license.name MIT
// @license.url https://opensource.org/licenses/MIT

// @host localhost:8080
// @BasePath /api/v1

func main() {
	// Initialize services
	sovdService := services.NewSOVDService()

	// Initialize handlers
	sovdHandler := handlers.NewSOVDHandler(sovdService)

	// Initialize Gin router
	router := gin.Default()

	// Add CORS middleware
	router.Use(func(c *gin.Context) {
		c.Header("Access-Control-Allow-Origin", "*")
		c.Header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		c.Header("Access-Control-Allow-Headers", "Origin, Content-Type, Content-Length, Accept-Encoding, X-CSRF-Token, Authorization")

		if c.Request.Method == "OPTIONS" {
			c.AbortWithStatus(204)
			return
		}

		c.Next()
	})

	// Health check endpoint
	router.GET("/health", sovdHandler.HealthCheck)

	// API v1 routes
	v1 := router.Group("/api/v1")
	{
		// Components routes - Data retrieval
		v1.GET("/components", sovdHandler.GetComponents)
		v1.GET("/components/:component_id/data", sovdHandler.GetComponentData)
		v1.GET("/components/:component_id/data/:data_id", sovdHandler.GetDataItemValue)
		
		// Components routes - Actuator control
		v1.POST("/components/:component_id/actuators/control", sovdHandler.ControlActuator)
		
		// Components routes - DTC management
		v1.POST("/components/:component_id/dtcs", sovdHandler.ManageDTCs)
		
		// Components routes - Generic services
		v1.POST("/components/:component_id/services", sovdHandler.ExecuteService)
	}

	// Add a root endpoint that shows API information
	router.GET("/", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"name":        "SOVD Server",
			"version":     "1.0.0",
			"description": "Service Oriented Vehicle Diagnostics (SOVD) Server API",
			"endpoints": gin.H{
				"health":                                "/health",
				"components":                            "/api/v1/components",
				"component_data":                        "/api/v1/components/{component_id}/data",
				"component_data_with_categories":        "/api/v1/components/{component_id}/data?categories={categories}",
				"specific_data_item":                    "/api/v1/components/{component_id}/data/{data_id}",
			},
			"example_requests": gin.H{
				"get_all_components":           "GET /api/v1/components",
				"get_engine_ident_data":        "GET /api/v1/components/engine/data?categories=identData",
				"get_vin_from_engine":          "GET /api/v1/components/engine/data/vin",
				"get_all_engine_data":          "GET /api/v1/components/engine/data",
				"get_transmission_sw_version":  "GET /api/v1/components/transmission/data/swversion",
				"control_engine_actuator":      "POST /api/v1/components/engine/actuators/control",
				"manage_engine_dtcs":           "POST /api/v1/components/engine/dtcs",
				"execute_diagnostic_routine":   "POST /api/v1/components/engine/services",
			},
		})
	})

	// Start the server
	log.Println("Starting SOVD Server on :8080")
	log.Println("API Documentation: http://localhost:8080/")
	log.Println("Health Check: http://localhost:8080/health")
	log.Println("Example VIN Request: http://localhost:8080/api/v1/components/engine/data/vin")

	if err := router.Run(":8080"); err != nil {
		log.Fatal("Failed to start server:", err)
	}
}
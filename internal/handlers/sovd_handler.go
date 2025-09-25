package handlers

import (
	"net/http"
	"sovd-server/internal/models"
	"sovd-server/internal/services"
	"strings"

	"github.com/gin-gonic/gin"
)

// SOVDHandler handles SOVD API requests
type SOVDHandler struct {
	service *services.SOVDService
}

// NewSOVDHandler creates a new SOVD handler
func NewSOVDHandler(service *services.SOVDService) *SOVDHandler {
	return &SOVDHandler{
		service: service,
	}
}

// GetComponents godoc
// @Summary Get all available components
// @Description Retrieve a list of all available vehicle components
// @Tags components
// @Accept json
// @Produce json
// @Success 200 {object} models.ComponentsResponse
// @Failure 500 {object} models.ErrorResponse
// @Router /components [get]
func (h *SOVDHandler) GetComponents(c *gin.Context) {
	components, err := h.service.GetComponents()
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.ErrorResponse{
			Error:   "Failed to retrieve components",
			Code:    http.StatusInternalServerError,
			Details: err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, models.ComponentsResponse{
		Components: components,
	})
}

// GetComponentData godoc
// @Summary Get component data items
// @Description Retrieve available data items for a specific component, optionally filtered by categories
// @Tags components
// @Accept json
// @Produce json
// @Param component_id path string true "Component ID" example("engine")
// @Param categories query string false "Filter by categories (comma-separated)" example("identData")
// @Success 200 {object} models.DataItemsResponse
// @Failure 404 {object} models.ErrorResponse
// @Failure 500 {object} models.ErrorResponse
// @Router /components/{component_id}/data [get]
func (h *SOVDHandler) GetComponentData(c *gin.Context) {
	componentID := c.Param("component_id")
	if componentID == "" {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Missing component_id parameter",
			Code:    http.StatusBadRequest,
			Details: "component_id path parameter is required",
		})
		return
	}

	// Parse categories from query parameter
	var categories []string
	if categoriesParam := c.Query("categories"); categoriesParam != "" {
		categories = strings.Split(categoriesParam, ",")
		// Trim whitespace from each category
		for i, category := range categories {
			categories[i] = strings.TrimSpace(category)
		}
	}

	items, err := h.service.GetComponentDataItems(componentID, categories)
	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			c.JSON(http.StatusNotFound, models.ErrorResponse{
				Error:   "Component not found",
				Code:    http.StatusNotFound,
				Details: err.Error(),
			})
			return
		}
		c.JSON(http.StatusInternalServerError, models.ErrorResponse{
			Error:   "Failed to retrieve component data items",
			Code:    http.StatusInternalServerError,
			Details: err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, models.DataItemsResponse{
		Items: items,
	})
}

// GetDataItemValue godoc
// @Summary Get specific data item value
// @Description Retrieve the value of a specific data item from a component
// @Tags components
// @Accept json
// @Produce json
// @Param component_id path string true "Component ID" example("engine")
// @Param data_id path string true "Data Item ID" example("vin")
// @Success 200 {object} models.DataItemValue
// @Failure 404 {object} models.ErrorResponse
// @Failure 500 {object} models.ErrorResponse
// @Router /components/{component_id}/data/{data_id} [get]
func (h *SOVDHandler) GetDataItemValue(c *gin.Context) {
	componentID := c.Param("component_id")
	dataID := c.Param("data_id")

	if componentID == "" {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Missing component_id parameter",
			Code:    http.StatusBadRequest,
			Details: "component_id path parameter is required",
		})
		return
	}

	if dataID == "" {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Missing data_id parameter",
			Code:    http.StatusBadRequest,
			Details: "data_id path parameter is required",
		})
		return
	}

	dataValue, err := h.service.GetDataItemValue(componentID, dataID)
	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			c.JSON(http.StatusNotFound, models.ErrorResponse{
				Error:   "Component or data item not found",
				Code:    http.StatusNotFound,
				Details: err.Error(),
			})
			return
		}
		c.JSON(http.StatusInternalServerError, models.ErrorResponse{
			Error:   "Failed to retrieve data item value",
			Code:    http.StatusInternalServerError,
			Details: err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, dataValue)
}

// ControlActuator godoc
// @Summary Control component actuator
// @Description Control an actuator on a specific component
// @Tags actuators
// @Accept json
// @Produce json
// @Param component_id path string true "Component ID" example("engine")
// @Param request body models.ActuatorControlRequest true "Actuator control request"
// @Success 200 {object} models.ActuatorControlResponse
// @Failure 400 {object} models.ErrorResponse
// @Failure 404 {object} models.ErrorResponse
// @Failure 500 {object} models.ErrorResponse
// @Router /components/{component_id}/actuators/control [post]
func (h *SOVDHandler) ControlActuator(c *gin.Context) {
	componentID := c.Param("component_id")
	if componentID == "" {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Missing component_id parameter",
			Code:    http.StatusBadRequest,
			Details: "component_id path parameter is required",
		})
		return
	}

	var request models.ActuatorControlRequest
	if err := c.ShouldBindJSON(&request); err != nil {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Invalid request body",
			Code:    http.StatusBadRequest,
			Details: err.Error(),
		})
		return
	}

	response, err := h.service.ControlActuator(componentID, &request)
	if err != nil {
		if strings.Contains(err.Error(), "not found") {
			c.JSON(http.StatusNotFound, models.ErrorResponse{
				Error:   "Component or actuator not found",
				Code:    http.StatusNotFound,
				Details: err.Error(),
			})
			return
		}
		c.JSON(http.StatusInternalServerError, models.ErrorResponse{
			Error:   "Failed to control actuator",
			Code:    http.StatusInternalServerError,
			Details: err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, response)
}

// ManageDTCs godoc
// @Summary Manage diagnostic trouble codes
// @Description Manage DTCs for a specific component (clear, read, freeze frame)
// @Tags diagnostics
// @Accept json
// @Produce json
// @Param component_id path string true "Component ID" example("engine")
// @Param request body models.DTCManagementRequest true "DTC management request"
// @Success 200 {object} models.DTCManagementResponse
// @Failure 400 {object} models.ErrorResponse
// @Failure 404 {object} models.ErrorResponse
// @Failure 500 {object} models.ErrorResponse
// @Router /components/{component_id}/dtcs [post]
func (h *SOVDHandler) ManageDTCs(c *gin.Context) {
	componentID := c.Param("component_id")
	if componentID == "" {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Missing component_id parameter",
			Code:    http.StatusBadRequest,
			Details: "component_id path parameter is required",
		})
		return
	}

	var request models.DTCManagementRequest
	if err := c.ShouldBindJSON(&request); err != nil {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Invalid request body",
			Code:    http.StatusBadRequest,
			Details: err.Error(),
		})
		return
	}

	response, err := h.service.ManageDTCs(componentID, &request)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.ErrorResponse{
			Error:   "Failed to manage DTCs",
			Code:    http.StatusInternalServerError,
			Details: err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, response)
}

// ExecuteService godoc
// @Summary Execute generic service request
// @Description Execute a generic service request on a component
// @Tags services
// @Accept json
// @Produce json
// @Param component_id path string true "Component ID" example("engine")
// @Param request body models.ServiceRequest true "Service request"
// @Success 200 {object} models.ServiceResponse
// @Failure 400 {object} models.ErrorResponse
// @Failure 404 {object} models.ErrorResponse
// @Failure 500 {object} models.ErrorResponse
// @Router /components/{component_id}/services [post]
func (h *SOVDHandler) ExecuteService(c *gin.Context) {
	componentID := c.Param("component_id")
	if componentID == "" {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Missing component_id parameter",
			Code:    http.StatusBadRequest,
			Details: "component_id path parameter is required",
		})
		return
	}

	var request models.ServiceRequest
	if err := c.ShouldBindJSON(&request); err != nil {
		c.JSON(http.StatusBadRequest, models.ErrorResponse{
			Error:   "Invalid request body",
			Code:    http.StatusBadRequest,
			Details: err.Error(),
		})
		return
	}

	response, err := h.service.ExecuteService(componentID, &request)
	if err != nil {
		c.JSON(http.StatusInternalServerError, models.ErrorResponse{
			Error:   "Failed to execute service",
			Code:    http.StatusInternalServerError,
			Details: err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, response)
}

// HealthCheck godoc
// @Summary Health check endpoint
// @Description Check if the SOVD server is running
// @Tags health
// @Accept json
// @Produce json
// @Success 200 {object} map[string]string
// @Router /health [get]
func (h *SOVDHandler) HealthCheck(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":  "ok",
		"service": "SOVD Server",
		"version": "1.0.0",
	})
}
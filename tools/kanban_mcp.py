#!/usr/bin/env python3
r"""
MCP Server para conectar Cortex con Planka (Kanban)
Ubicacion: E:\scripts-python\cortex\tools\kanban_mcp.py
"""

import json
import sys
import requests
from typing import Any, Dict, List, Optional

# Configuración
PLANKA_API_URL = "http://localhost:3000"
TIMEOUT = 10


def make_request(method: str, endpoint: str, data: Optional[Dict] = None) -> Dict[str, Any]:
    """Hace request a la API de Planka"""
    url = f"{PLANKA_API_URL}{endpoint}"
    headers = {"Content-Type": "application/json"}
    
    try:
        if method.upper() == "GET":
            response = requests.get(url, headers=headers, timeout=TIMEOUT)
        elif method.upper() == "POST":
            response = requests.post(url, json=data, headers=headers, timeout=TIMEOUT)
        elif method.upper() == "PUT":
            response = requests.put(url, json=data, headers=headers, timeout=TIMEOUT)
        elif method.upper() == "DELETE":
            response = requests.delete(url, headers=headers, timeout=TIMEOUT)
        else:
            return {"error": f"Método {method} no soportado"}
        
        response.raise_for_status()
        return response.json() if response.content else {"success": True}
    except requests.exceptions.ConnectionError:
        return {"error": "No se puede conectar a Planka. ¿Está ejecutándose en localhost:3000?"}
    except requests.exceptions.Timeout:
        return {"error": "Timeout al conectar con Planka"}
    except requests.exceptions.RequestException as e:
        return {"error": f"Error de request: {str(e)}"}
    except json.JSONDecodeError:
        return {"error": "Respuesta inválida de Planka"}


# === Herramientas MCP ===

def list_boards() -> Dict[str, Any]:
    """Lista todos los boards disponibles"""
    result = make_request("GET", "/api/boards")
    if "error" in result:
        return result
    
    # Normalizar respuesta
    boards = result if isinstance(result, list) else result.get("items", result.get("data", []))
    
    boards_list = []
    for board in boards:
        boards_list.append({
            "id": board.get("id"),
            "name": board.get("name"),
            "description": board.get("description"),
        })
    
    return {"boards": boards_list, "count": len(boards_list)}


def list_tasks(board_id: str) -> Dict[str, Any]:
    """Lista todas las tareas de un board específico"""
    if not board_id:
        return {"error": "board_id es requerido"}
    
    result = make_request("GET", f"/api/boards/{board_id}/tasks")
    if "error" in result:
        return result
    
    # Normalizar respuesta
    tasks = result if isinstance(result, list) else result.get("items", result.get("data", []))
    
    tasks_list = []
    for task in tasks:
        tasks_list.append({
            "id": task.get("id"),
            "name": task.get("name"),
            "description": task.get("description"),
            "column_id": task.get("columnId"),
            "row": task.get("row"),
        })
    
    return {"tasks": tasks_list, "count": len(tasks_list)}


def create_task(board_id: str, column_id: str, name: str, description: str = "") -> Dict[str, Any]:
    """Crea una tarea en un board y columna específicos"""
    if not board_id:
        return {"error": "board_id es requerido"}
    if not column_id:
        return {"error": "column_id es requerido"}
    if not name:
        return {"error": "name es requerido"}
    
    data = {
        "name": name,
        "description": description,
        "columnId": column_id,
    }
    
    result = make_request("POST", f"/api/boards/{board_id}/tasks", data)
    if "error" in result:
        return result
    
    return {
        "success": True,
        "task": {
            "id": result.get("id"),
            "name": result.get("name"),
            "description": result.get("description"),
            "column_id": result.get("columnId"),
        }
    }


def move_task(task_id: str, column_id: str, row: int = 0) -> Dict[str, Any]:
    """Mueve una tarea a una columna específica"""
    if not task_id:
        return {"error": "task_id es requerido"}
    if not column_id:
        return {"error": "column_id es requerido"}
    
    data = {
        "columnId": column_id,
        "row": row,
    }
    
    result = make_request("PUT", f"/api/tasks/{task_id}", data)
    if "error" in result:
        return result
    
    return {
        "success": True,
        "task": {
            "id": result.get("id"),
            "name": result.get("name"),
            "column_id": result.get("columnId"),
            "row": result.get("row"),
        }
    }


# === Protocolo MCP (JSON-RPC sobre stdio) ===

def handle_request(request: Dict[str, Any]) -> Dict[str, Any]:
    """Procesa una request MCP"""
    method = request.get("method", "")
    params = request.get("params", {})
    request_id = request.get("id")
    
    result = None
    error = None
    
    try:
        if method == "tools/list":
            result = {
                "tools": [
                    {
                        "name": "list_boards",
                        "description": "Lista todos los boards disponibles en Planka",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    },
                    {
                        "name": "list_tasks",
                        "description": "Lista todas las tareas de un board específico",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "board_id": {"type": "string", "description": "ID del board"}
                            },
                            "required": ["board_id"]
                        }
                    },
                    {
                        "name": "create_task",
                        "description": "Crea una nueva tarea en un board y columna",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "board_id": {"type": "string", "description": "ID del board"},
                                "column_id": {"type": "string", "description": "ID de la columna"},
                                "name": {"type": "string", "description": "Nombre de la tarea"},
                                "description": {"type": "string", "description": "Descripción de la tarea"}
                            },
                            "required": ["board_id", "column_id", "name"]
                        }
                    },
                    {
                        "name": "move_task",
                        "description": "Mueve una tarea a una columna diferente",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "task_id": {"type": "string", "description": "ID de la tarea"},
                                "column_id": {"type": "string", "description": "ID de la columna destino"},
                                "row": {"type": "integer", "description": "Posición en la columna", "default": 0}
                            },
                            "required": ["task_id", "column_id"]
                        }
                    }
                ]
            }
        elif method == "tools/call":
            tool_name = params.get("name")
            tool_args = params.get("arguments", {})
            
            if tool_name == "list_boards":
                result = list_boards()
            elif tool_name == "list_tasks":
                result = list_tasks(tool_args.get("board_id", ""))
            elif tool_name == "create_task":
                result = create_task(
                    tool_args.get("board_id", ""),
                    tool_args.get("column_id", ""),
                    tool_args.get("name", ""),
                    tool_args.get("description", "")
                )
            elif tool_name == "move_task":
                result = move_task(
                    tool_args.get("task_id", ""),
                    tool_args.get("column_id", ""),
                    tool_args.get("row", 0)
                )
            else:
                error = {"code": -32601, "message": f"Tool '{tool_name}' no encontrada"}
        else:
            error = {"code": -32601, "message": f"Método '{method}' no soportado"}
    
    except Exception as e:
        error = {"code": -32603, "message": f"Error interno: {str(e)}"}
    
    response = {"jsonrpc": "2.0", "id": request_id}
    
    if error:
        response["error"] = error
    else:
        response["result"] = result
    
    return response


def main():
    """Loop principal - lee JSON-RPC de stdin y responde por stdout"""
    while True:
        try:
            line = sys.stdin.readline()
            if not line:
                break
            
            line = line.strip()
            if not line:
                continue
            
            request = json.loads(line)
            response = handle_request(request)
            print(json.dumps(response), flush=True)
            
        except json.JSONDecodeError as e:
            error_response = {
                "jsonrpc": "2.0",
                "error": {"code": -32700, "message": f"Parse error: {str(e)}"},
                "id": None
            }
            print(json.dumps(error_response), flush=True)
        except Exception as e:
            error_response = {
                "jsonrpc": "2.0",
                "error": {"code": -32603, "message": f"Internal error: {str(e)}"},
                "id": None
            }
            print(json.dumps(error_response), flush=True)


if __name__ == "__main__":
    main()

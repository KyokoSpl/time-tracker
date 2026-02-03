/**
 * Time Tracker Tauri Application
 * 
 * JavaScript frontend logic for communicating with the Rust backend
 * and managing the UI state.
 */

// Application State
let tasks = [];
let updateInterval = null;
let taskToReset = null;
let taskToDelete = null;
let isDarkMode = true;

// DOM Elements - initialized after DOM loads
let taskInput;
let addTaskBtn;
let tasksContainer;
let emptyState;
let themeToggle;
let themeIcon;
let exportBtn;
let resetDialog;
let deleteDialog;
let resetMessage;
let deleteMessage;
let confirmResetBtn;
let confirmDeleteBtn;
let snackbar;
let snackbarIcon;
let snackbarMessage;

/**
 * Waits for Tauri API to be available
 * @returns {Promise<void>}
 */
function waitForTauri() {
    return new Promise((resolve) => {
        if (window.__TAURI__) {
            resolve();
            return;
        }
        
        // Poll for Tauri API
        const checkInterval = setInterval(() => {
            if (window.__TAURI__) {
                clearInterval(checkInterval);
                resolve();
            }
        }, 50);
        
        // Timeout after 5 seconds
        setTimeout(() => {
            clearInterval(checkInterval);
            resolve(); // Resolve anyway, will show error in UI
        }, 5000);
    });
}

/**
 * Invokes a Tauri command safely.
 * @param {string} cmd - Command name
 * @param {object} args - Command arguments
 * @returns {Promise<any>} - Command result
 */
async function invoke(cmd, args = {}) {
    await waitForTauri();
    
    if (window.__TAURI__) {
        // Tauri v2 API structure
        if (window.__TAURI__.core && window.__TAURI__.core.invoke) {
            return window.__TAURI__.core.invoke(cmd, args);
        }
        // Alternative path
        if (window.__TAURI__.tauri && window.__TAURI__.tauri.invoke) {
            return window.__TAURI__.tauri.invoke(cmd, args);
        }
        // Direct invoke (older API)
        if (window.__TAURI__.invoke) {
            return window.__TAURI__.invoke(cmd, args);
        }
    }
    
    throw new Error('Tauri API not available');
}

/**
 * Opens a save dialog using Tauri dialog plugin.
 * @param {object} options - Dialog options
 * @returns {Promise<string|null>} - Selected file path or null
 */
async function saveDialog(options) {
    await waitForTauri();
    
    if (window.__TAURI__ && window.__TAURI__.dialog) {
        return window.__TAURI__.dialog.save(options);
    }
    
    throw new Error('Tauri dialog API not available');
}

/**
 * Initializes the application when DOM is ready.
 */
async function init() {
    console.log('Initializing Time Tracker...');
    
    // Initialize DOM references
    taskInput = document.getElementById('task-input');
    addTaskBtn = document.getElementById('add-task-btn');
    tasksContainer = document.getElementById('tasks-container');
    emptyState = document.getElementById('empty-state');
    themeToggle = document.getElementById('theme-toggle');
    themeIcon = document.getElementById('theme-icon');
    exportBtn = document.getElementById('export-btn');
    resetDialog = document.getElementById('reset-dialog');
    deleteDialog = document.getElementById('delete-dialog');
    resetMessage = document.getElementById('reset-message');
    deleteMessage = document.getElementById('delete-message');
    confirmResetBtn = document.getElementById('confirm-reset-btn');
    confirmDeleteBtn = document.getElementById('confirm-delete-btn');
    snackbar = document.getElementById('snackbar');
    snackbarIcon = document.getElementById('snackbar-icon');
    snackbarMessage = document.getElementById('snackbar-message');

    // Wait for Tauri to be ready
    console.log('Waiting for Tauri API...');
    await waitForTauri();
    
    if (window.__TAURI__) {
        console.log('Tauri API available:', Object.keys(window.__TAURI__));
    } else {
        console.error('Tauri API not available after waiting');
        showSnackbar('Error: Tauri API not available', 'error');
        return;
    }

    // Set up event listeners
    setupEventListeners();

    // Load initial tasks
    await loadTasks();

    // Start the update interval for running tasks
    startUpdateInterval();
    
    console.log('Time Tracker initialized successfully');
}

/**
 * Sets up all event listeners for the application.
 */
function setupEventListeners() {
    // Add task on button click
    addTaskBtn.addEventListener('click', handleAddTask);

    // Add task on Enter key
    taskInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            handleAddTask();
        }
    });

    // Theme toggle
    themeToggle.addEventListener('click', toggleTheme);

    // Export button
    exportBtn.addEventListener('click', handleExport);

    // Dialog confirmations
    confirmResetBtn.addEventListener('click', confirmReset);
    confirmDeleteBtn.addEventListener('click', confirmDelete);
}

/**
 * Loads all tasks from the backend.
 */
async function loadTasks() {
    try {
        tasks = await invoke('get_tasks');
        renderTasks();
    } catch (error) {
        console.error('Error loading tasks:', error);
        showSnackbar(`Error loading tasks: ${error}`, 'error');
    }
}

/**
 * Renders all tasks in the UI.
 */
function renderTasks() {
    // Clear existing task cards (keep empty state)
    const existingCards = tasksContainer.querySelectorAll('.task-card');
    existingCards.forEach(card => card.remove());

    if (tasks.length === 0) {
        emptyState.style.display = 'flex';
        return;
    }

    emptyState.style.display = 'none';

    tasks.forEach(task => {
        const card = createTaskCard(task);
        tasksContainer.appendChild(card);
    });
}

/**
 * Creates a task card element.
 * @param {Object} task - Task data object
 * @returns {HTMLElement} - The task card element
 */
function createTaskCard(task) {
    const card = document.createElement('article');
    card.className = `task-card ${task.is_running ? 'running' : ''}`;
    card.dataset.taskName = task.name;

    const row = document.createElement('div');
    row.className = 'task-row';

    // Task info section (name + time)
    const taskInfo = document.createElement('div');
    taskInfo.className = 'task-info';

    const taskName = document.createElement('span');
    taskName.className = 'task-name';
    taskName.textContent = task.name;
    taskName.title = task.name;

    const timeDisplay = document.createElement('span');
    timeDisplay.className = 'time-display';
    timeDisplay.textContent = task.formatted_time;

    taskInfo.appendChild(taskName);
    taskInfo.appendChild(timeDisplay);

    // Running indicator
    if (task.is_running) {
        const indicator = document.createElement('span');
        indicator.className = 'running-indicator';
        indicator.innerHTML = '<i>radio_button_checked</i>';
        taskInfo.appendChild(indicator);
    }

    // Action buttons
    const actions = document.createElement('div');
    actions.className = 'task-actions';

    // Start/Stop button
    const toggleBtn = document.createElement('button');
    toggleBtn.className = `circle ${task.is_running ? 'stop-btn' : 'start-btn'}`;
    toggleBtn.innerHTML = `<i>${task.is_running ? 'pause' : 'play_arrow'}</i>`;
    toggleBtn.title = task.is_running ? 'Stop' : 'Start';
    toggleBtn.addEventListener('click', () => handleToggleTask(task.name, task.is_running));

    // Reset button
    const resetBtn = document.createElement('button');
    resetBtn.className = 'circle reset-btn';
    resetBtn.innerHTML = '<i>replay</i>';
    resetBtn.title = 'Reset time';
    resetBtn.addEventListener('click', () => showResetDialog(task.name));

    // Delete button
    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'circle delete-btn';
    deleteBtn.innerHTML = '<i>delete</i>';
    deleteBtn.title = 'Delete task';
    deleteBtn.addEventListener('click', () => showDeleteDialog(task.name));

    actions.appendChild(toggleBtn);
    actions.appendChild(resetBtn);
    actions.appendChild(deleteBtn);

    row.appendChild(taskInfo);
    row.appendChild(actions);
    card.appendChild(row);

    return card;
}

/**
 * Handles adding a new task.
 */
async function handleAddTask() {
    const name = taskInput.value.trim();

    if (!name) {
        showSnackbar('Please enter a task name', 'warning');
        taskInput.focus();
        return;
    }

    try {
        await invoke('add_task', { name });
        taskInput.value = '';
        await loadTasks();
        showSnackbar(`Task "${name}" added`, 'success');
    } catch (error) {
        showSnackbar(String(error), 'error');
    }
}

/**
 * Handles starting or stopping a task.
 * @param {string} name - Task name
 * @param {boolean} isRunning - Current running state
 */
async function handleToggleTask(name, isRunning) {
    try {
        if (isRunning) {
            await invoke('stop_task', { name });
        } else {
            await invoke('start_task', { name });
        }
        await loadTasks();
    } catch (error) {
        showSnackbar(`Error: ${error}`, 'error');
    }
}

/**
 * Shows the reset confirmation dialog.
 * @param {string} name - Task name to reset
 */
function showResetDialog(name) {
    taskToReset = name;
    resetMessage.textContent = `Are you sure you want to reset the time for "${name}"?`;
    if (typeof ui === 'function') {
        ui('#reset-dialog');
    } else {
        resetDialog.showModal();
    }
}

/**
 * Confirms and executes the reset action.
 */
async function confirmReset() {
    if (!taskToReset) return;

    try {
        await invoke('reset_task', { name: taskToReset });
        if (typeof ui === 'function') {
            ui('#reset-dialog');
        } else {
            resetDialog.close();
        }
        await loadTasks();
        showSnackbar(`Task "${taskToReset}" reset`, 'success');
    } catch (error) {
        showSnackbar(`Error: ${error}`, 'error');
    }

    taskToReset = null;
}

/**
 * Shows the delete confirmation dialog.
 * @param {string} name - Task name to delete
 */
function showDeleteDialog(name) {
    taskToDelete = name;
    deleteMessage.textContent = `Are you sure you want to delete "${name}"?`;
    if (typeof ui === 'function') {
        ui('#delete-dialog');
    } else {
        deleteDialog.showModal();
    }
}

/**
 * Confirms and executes the delete action.
 */
async function confirmDelete() {
    if (!taskToDelete) return;

    try {
        await invoke('delete_task', { name: taskToDelete });
        if (typeof ui === 'function') {
            ui('#delete-dialog');
        } else {
            deleteDialog.close();
        }
        await loadTasks();
        showSnackbar(`Task "${taskToDelete}" deleted`, 'success');
    } catch (error) {
        showSnackbar(`Error: ${error}`, 'error');
    }

    taskToDelete = null;
}

/**
 * Handles exporting tasks to a file.
 */
async function handleExport() {
    try {
        const filePath = await saveDialog({
            filters: [{
                name: 'Text files',
                extensions: ['txt']
            }],
            defaultPath: 'time_tracker_export.txt'
        });

        if (filePath) {
            await invoke('export_tasks', { path: filePath });
            showSnackbar('Tasks exported successfully', 'success');
        }
    } catch (error) {
        showSnackbar(`Export failed: ${error}`, 'error');
    }
}

/**
 * Toggles between dark and light theme.
 */
function toggleTheme() {
    isDarkMode = !isDarkMode;
    document.body.className = isDarkMode ? 'dark' : 'light';
    themeIcon.textContent = isDarkMode ? 'light_mode' : 'dark_mode';
}

/**
 * Starts the interval to update running task times.
 */
function startUpdateInterval() {
    // Update every second
    updateInterval = setInterval(async () => {
        const hasRunningTasks = tasks.some(t => t.is_running);
        if (hasRunningTasks) {
            await loadTasks();
        }
    }, 1000);
}

/**
 * Shows a snackbar notification.
 * @param {string} message - Message to display
 * @param {string} type - Type of notification (info, success, error, warning)
 */
function showSnackbar(message, type = 'info') {
    // Update snackbar content
    snackbarMessage.textContent = message;

    // Set icon based on type
    const icons = {
        info: 'info',
        success: 'check_circle',
        error: 'error',
        warning: 'warning'
    };
    snackbarIcon.textContent = icons[type] || 'info';

    // Update styling
    snackbar.className = `snackbar ${type}`;

    // Show snackbar using BeerCSS ui function or fallback
    if (typeof ui === 'function') {
        ui('#snackbar', 4000);
    } else {
        // Fallback: manually show/hide
        snackbar.classList.add('active');
        setTimeout(() => {
            snackbar.classList.remove('active');
        }, 4000);
    }
}

/**
 * Formats seconds into HH:MM:SS format.
 * @param {number} totalSeconds - Total seconds
 * @returns {string} - Formatted time string
 */
function formatTime(totalSeconds) {
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
}

// Initialize the application when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    // DOM already loaded, wait a bit for Tauri to inject its API
    setTimeout(init, 100);
}

// Clean up on window unload
window.addEventListener('beforeunload', () => {
    if (updateInterval) {
        clearInterval(updateInterval);
    }
});
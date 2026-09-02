<script setup lang="ts">
/**
 * Back-office : catégories de ventilation des points (daisyUI autorisé ici).
 *
 * La catégorie d'une règle est **recopiée sur chaque mouvement à l'écriture** :
 * re-catégoriser une règle ne déplace donc aucun point déjà gagné. C'est voulu 
 * la ventilation reflète la catégorie au moment du mouvement.
 */
import { ref, onMounted } from 'vue'
import { useAdminEngagement, type AdminCategorie } from '~/composables/useAdminEngagement'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerCategories, creerCategorie, modifierCategorie, supprimerCategorie } =
  useAdminEngagement()

const categories = ref<AdminCategorie[]>([])
const chargement = ref(true)
const message = ref('')
const erreur = ref('')

const formulaire = ref(false)
const nouvelle = ref({
  code: '', libelle: '', description: '', ordre: 0, couleur: '', icone: '',
})

/** Jetons de couleur reconnus par les composants membres (`VentilationCategories`). */
const COULEURS = ['green', 'rose', 'amber', 'sky', 'violet', 'gray']

const rafraichir = async () => {
  chargement.value = true
  categories.value = await listerCategories()
  chargement.value = false
}

onMounted(rafraichir)

const notifier = (m: string) => {
  message.value = m
  erreur.value = ''
  setTimeout(() => { message.value = '' }, 2500)
}

const signaler = (e: unknown) => {
  message.value = ''
  const data = (e as { data?: { error?: string } })?.data
  erreur.value = data?.error || 'L\'opération a échoué.'
}

const ajouter = async () => {
  const f = nouvelle.value
  if (!f.code.trim() || !f.libelle.trim()) {
    erreur.value = 'Le code et le libellé sont obligatoires.'
    return
  }
  try {
    await creerCategorie({
      code: f.code.trim(),
      libelle: f.libelle.trim(),
      description: f.description.trim() || null,
      ordre: f.ordre,
      couleur: f.couleur || null,
      icone: f.icone.trim() || null,
    })
    formulaire.value = false
    nouvelle.value = { code: '', libelle: '', description: '', ordre: 0, couleur: '', icone: '' }
    await rafraichir()
    notifier('Catégorie créée')
  } catch (e) { signaler(e) }
}

const enregistrer = async (c: AdminCategorie) => {
  try {
    await modifierCategorie(c.id, {
      libelle: c.libelle,
      description: c.description,
      ordre: c.ordre,
      couleur: c.couleur,
      icone: c.icone,
      actif: c.actif,
    })
    notifier('Catégorie enregistrée')
  } catch (e) { signaler(e) }
}

/**
 * Avertissement chiffré avant suppression : les règles bloquent (409 serveur),
 * les mouvements non : ils basculeraient simplement sous « Autres » chez leurs
 * détenteurs, ce que l'administrateur doit savoir avant de confirmer.
 */
const supprimer = async (c: AdminCategorie) => {
  if (c.nombre_regles > 0) {
    erreur.value = `${c.nombre_regles} règle(s) utilisent « ${c.libelle} » : réaffectez-les d'abord, ou désactivez la catégorie.`
    return
  }
  const avertissement = c.nombre_mouvements > 0
    ? `\n\n⚠️ ${c.nombre_mouvements} mouvement(s) de points portent cette catégorie : ils basculeront sous « Autres » dans l'espace des membres concernés.`
    : ''
  if (!confirm(`Supprimer la catégorie « ${c.libelle} » ?${avertissement}`)) return
  try {
    await supprimerCategorie(c.id)
    await rafraichir()
    notifier('Catégorie supprimée')
  } catch (e) { signaler(e) }
}
</script>

<template>
  <div class="p-6 space-y-6">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-bold">Engagement, Catégories de points</h1>
      <div class="flex flex-wrap gap-2">
        <NuxtLink to="/admin/engagement/regles" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-sliders" /> Barème
        </NuxtLink>
        <button class="btn btn-sm btn-primary" @click="formulaire = !formulaire">
          <font-awesome-icon :icon="formulaire ? 'fa-solid fa-xmark' : 'fa-solid fa-plus'" />
          {{ formulaire ? 'Annuler' : 'Nouvelle catégorie' }}
        </button>
      </div>
    </div>

    <p class="text-sm opacity-70">
      Les catégories servent la ventilation « d'où viennent mes points » dans l'espace membre.
      La catégorie est figée sur chaque mouvement à sa création : re-catégoriser une règle
      n'affecte donc <strong>que les points futurs</strong>.
    </p>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error py-2">{{ erreur }}</div>
    <div v-if="chargement" class="loading loading-spinner" />

    <template v-else>
      <div v-if="formulaire" class="card bg-base-200">
        <div class="card-body gap-3">
          <h2 class="font-semibold">Créer une catégorie</h2>
          <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
            <label class="form-control">
              <span class="label-text text-xs">Code (immuable ensuite)</span>
              <input
                v-model="nouvelle.code"
                class="input input-sm input-bordered font-mono"
                placeholder="ex. mentorat"
              >
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Libellé</span>
              <input v-model="nouvelle.libelle" class="input input-sm input-bordered" placeholder="ex. Mentorat">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Ordre d'affichage</span>
              <input v-model.number="nouvelle.ordre" type="number" class="input input-sm input-bordered">
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Couleur</span>
              <select v-model="nouvelle.couleur" class="select select-sm select-bordered">
                <option value="">par défaut</option>
                <option v-for="c in COULEURS" :key="c" :value="c">{{ c }}</option>
              </select>
            </label>
            <label class="form-control">
              <span class="label-text text-xs">Icône (nom FontAwesome)</span>
              <input v-model="nouvelle.icone" class="input input-sm input-bordered font-mono" placeholder="ex. handshake">
            </label>
            <label class="form-control md:col-span-3">
              <span class="label-text text-xs">Description (affichée dans l'état vide de l'espace membre)</span>
              <textarea v-model="nouvelle.description" rows="2" class="textarea textarea-sm textarea-bordered" />
            </label>
          </div>
          <div class="card-actions justify-end">
            <button class="btn btn-sm btn-success" @click="ajouter">Créer</button>
          </div>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="table table-zebra table-sm">
          <thead>
            <tr>
              <th>Code</th><th>Libellé</th><th>Ordre</th><th>Couleur</th><th>Icône</th>
              <th>Description</th><th>Règles</th><th>Mouvements</th><th>Actif</th><th />
            </tr>
          </thead>
          <tbody>
            <tr v-for="c in categories" :key="c.id">
              <td class="font-mono text-xs">{{ c.code }}</td>
              <td><input v-model="c.libelle" class="input input-sm input-bordered w-40"></td>
              <td><input v-model.number="c.ordre" type="number" class="input input-sm input-bordered w-16"></td>
              <td>
                <select v-model="c.couleur" class="select select-sm select-bordered w-28">
                  <option :value="null">-</option>
                  <option v-for="col in COULEURS" :key="col" :value="col">{{ col }}</option>
                </select>
              </td>
              <td><input v-model="c.icone" class="input input-sm input-bordered w-32 font-mono"></td>
              <td><textarea v-model="c.description" rows="2" class="textarea textarea-sm textarea-bordered w-56" /></td>
              <td class="text-center text-xs">{{ c.nombre_regles }}</td>
              <td class="text-center text-xs">{{ c.nombre_mouvements }}</td>
              <td><input v-model="c.actif" type="checkbox" class="toggle toggle-sm"></td>
              <td class="flex gap-1">
                <button class="btn btn-xs btn-primary" @click="enregistrer(c)">Enregistrer</button>
                <button
                  class="btn btn-xs btn-error btn-outline"
                  :disabled="c.nombre_regles > 0"
                  :title="c.nombre_regles > 0 ? 'Des règles utilisent cette catégorie' : 'Supprimer'"
                  @click="supprimer(c)"
                >
                  <font-awesome-icon icon="fa-solid fa-trash" />
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>
  </div>
</template>

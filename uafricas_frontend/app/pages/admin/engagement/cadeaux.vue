<script setup lang="ts">
/**
 * Back-office — catalogue des cadeaux virtuels et paramètres de monétisation.
 *
 * daisyUI est **autorisé ici** : le Principe VI le réserve à l'administration,
 * les écrans membres restant en Tailwind v4 pur.
 */
import { computed, onMounted, ref } from 'vue'
import {
  useAdminCadeaux,
  type AdminCadeau,
  type AdminParametresMonetisation,
} from '~/composables/useAdminCadeaux'
import { formaterMontant } from '~/composables/useCadeaux'

definePageMeta({ layout: 'admin', middleware: ['admin'] })
useHead({ title: 'Cadeaux virtuels — Administration' })

const {
  listerCadeaux, creerCadeau, modifierCadeau, supprimerCadeau,
  obtenirParametres, modifierParametres,
} = useAdminCadeaux()

const cadeaux = ref<AdminCadeau[]>([])
const parametres = ref<AdminParametresMonetisation | null>(null)
const chargement = ref(true)
const message = ref('')
const erreur = ref('')

const formulaire = ref(false)
const nouveau = ref({
  code: '', libelle: '', description: '', icone: 'gift', couleur: 'amber',
  prix: 500, points: 5, ordre: 0, actif: true,
})

const devise = computed(() => parametres.value?.devise ?? 'XOF')

const rafraichir = async () => {
  chargement.value = true
  ;[cadeaux.value, parametres.value] = await Promise.all([
    listerCadeaux(),
    obtenirParametres(),
  ])
  chargement.value = false
}

onMounted(rafraichir)

const notifier = (m: string) => {
  message.value = m
  erreur.value = ''
  setTimeout(() => { message.value = '' }, 4000)
}

/** Les messages 400/409 du serveur sont déjà rédigés en français : on les affiche tels quels. */
const signaler = (e: unknown) => {
  message.value = ''
  const data = (e as { data?: { error?: string } })?.data
  erreur.value = data?.error || 'L\'opération a échoué.'
}

const ajouter = async () => {
  const f = nouveau.value
  if (!f.code.trim() || !f.libelle.trim()) {
    erreur.value = 'Le code et le libellé sont obligatoires.'
    return
  }
  try {
    await creerCadeau({
      code: f.code.trim(),
      libelle: f.libelle.trim(),
      description: f.description.trim() || null,
      icone: f.icone.trim() || null,
      couleur: f.couleur.trim() || null,
      prix: f.prix,
      points: f.points,
      ordre: f.ordre,
      actif: f.actif,
    })
    nouveau.value = {
      code: '', libelle: '', description: '', icone: 'gift', couleur: 'amber',
      prix: 500, points: 5, ordre: 0, actif: true,
    }
    formulaire.value = false
    await rafraichir()
    notifier('Cadeau créé — il est immédiatement visible côté membre.')
  } catch (e) { signaler(e) }
}

const enregistrer = async (c: AdminCadeau) => {
  try {
    await modifierCadeau(c.id, {
      libelle: c.libelle,
      description: c.description,
      icone: c.icone,
      couleur: c.couleur,
      prix: c.prix,
      points: c.points,
      ordre: c.ordre,
      actif: c.actif,
    })
    notifier('Cadeau enregistré — les envois passés conservent leurs valeurs figées.')
  } catch (e) { signaler(e) }
}

const retirer = async (c: AdminCadeau) => {
  if (!confirm(`Supprimer définitivement « ${c.libelle} » ?`)) return
  try {
    await supprimerCadeau(c.id)
    await rafraichir()
    notifier('Cadeau supprimé')
  } catch (e) { signaler(e) }
}

const enregistrerParametres = async () => {
  if (!parametres.value) return
  try {
    parametres.value = await modifierParametres({
      taux_commission: parametres.value.taux_commission,
      devise: parametres.value.devise,
      paiement_reel_actif: parametres.value.paiement_reel_actif,
    })
    notifier('Paramètres enregistrés — la modification ne vaut que pour les envois à venir.')
  } catch (e) { signaler(e) }
}
</script>

<template>
  <div class="space-y-8 p-6">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-bold">Cadeaux virtuels</h1>
      <div class="flex flex-wrap gap-2">
        <NuxtLink to="/admin/engagement/transactions" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-coins" /> Journal comptable
        </NuxtLink>
        <NuxtLink to="/admin/engagement/regles" class="btn btn-sm btn-outline">
          <font-awesome-icon icon="fa-solid fa-sliders" /> Barème
        </NuxtLink>
      </div>
    </div>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error py-2">{{ erreur }}</div>
    <div v-if="chargement" class="loading loading-spinner" />

    <template v-else>
      <!-- ─── Catalogue ─── -->
      <section class="space-y-3">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <h2 class="text-lg font-semibold">Catalogue</h2>
          <button class="btn btn-sm btn-primary" @click="formulaire = !formulaire">
            <font-awesome-icon :icon="formulaire ? 'fa-solid fa-xmark' : 'fa-solid fa-plus'" />
            {{ formulaire ? 'Annuler' : 'Nouveau cadeau' }}
          </button>
        </div>

        <!--
          Le prix et les points sont figés sur chaque transaction à l'envoi :
          sans ce rappel, on croirait qu'ajuster un prix réécrit la comptabilité
          passée.
        -->
        <div class="alert alert-info py-2 text-sm">
          <font-awesome-icon icon="fa-solid fa-circle-info" />
          <span>
            Modifier le <strong>prix</strong> ou les <strong>points</strong> d'un cadeau
            n'affecte <strong>aucun envoi passé</strong> : chaque transaction porte ses propres
            valeurs, figées au moment de l'envoi. Un cadeau déjà offert ne peut plus être
            supprimé — seulement désactivé.
          </span>
        </div>

        <div v-if="formulaire" class="card bg-base-200">
          <div class="card-body gap-3">
            <h3 class="font-semibold">Créer un cadeau</h3>
            <div class="grid grid-cols-1 gap-3 md:grid-cols-2 lg:grid-cols-4">
              <label class="form-control">
                <span class="label-text text-xs">Code (clé stable, immuable)</span>
                <input v-model="nouveau.code" class="input input-sm input-bordered font-mono" placeholder="tam_tam">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Libellé</span>
                <input v-model="nouveau.libelle" class="input input-sm input-bordered" placeholder="Tam-tam">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Prix ({{ devise }})</span>
                <input v-model.number="nouveau.prix" type="number" min="1" class="input input-sm input-bordered">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Points crédités</span>
                <input v-model.number="nouveau.points" type="number" min="1" class="input input-sm input-bordered">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Icône FontAwesome</span>
                <input v-model="nouveau.icone" class="input input-sm input-bordered font-mono" placeholder="drum">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Couleur</span>
                <input v-model="nouveau.couleur" class="input input-sm input-bordered font-mono" placeholder="amber">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Ordre d'affichage</span>
                <input v-model.number="nouveau.ordre" type="number" class="input input-sm input-bordered">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Actif</span>
                <input v-model="nouveau.actif" type="checkbox" class="toggle toggle-sm mt-1">
              </label>
            </div>
            <label class="form-control">
              <span class="label-text text-xs">Description (facultative)</span>
              <input v-model="nouveau.description" class="input input-sm input-bordered">
            </label>
            <div class="card-actions justify-end">
              <button class="btn btn-sm btn-success" @click="ajouter">Créer le cadeau</button>
            </div>
          </div>
        </div>

        <div class="overflow-x-auto">
          <table class="table table-zebra table-sm">
            <thead>
              <tr>
                <th>Code</th><th>Libellé</th><th>Icône</th><th>Couleur</th>
                <th>Prix</th><th>Points</th><th>Ordre</th>
                <th>Envois</th><th>Collecté</th><th>Actif</th><th />
              </tr>
            </thead>
            <tbody>
              <tr v-for="c in cadeaux" :key="c.id" :class="{ 'opacity-60': !c.actif }">
                <td class="font-mono text-xs">{{ c.code }}</td>
                <td><input v-model="c.libelle" class="input input-sm input-bordered w-40"></td>
                <td><input v-model="c.icone" class="input input-sm input-bordered w-28 font-mono"></td>
                <td><input v-model="c.couleur" class="input input-sm input-bordered w-24 font-mono"></td>
                <td><input v-model.number="c.prix" type="number" min="1" class="input input-sm input-bordered w-24"></td>
                <td><input v-model.number="c.points" type="number" min="1" class="input input-sm input-bordered w-20"></td>
                <td><input v-model.number="c.ordre" type="number" class="input input-sm input-bordered w-16"></td>
                <td class="text-xs">{{ c.nombre_envois }}</td>
                <td class="whitespace-nowrap text-xs">{{ formaterMontant(c.montant_collecte, devise) }}</td>
                <td><input v-model="c.actif" type="checkbox" class="toggle toggle-sm"></td>
                <td class="flex gap-1">
                  <button class="btn btn-xs btn-primary" @click="enregistrer(c)">Enregistrer</button>
                  <button
                    class="btn btn-xs btn-error btn-outline"
                    :disabled="c.nombre_envois > 0"
                    :title="c.nombre_envois > 0
                      ? 'Cadeau déjà offert : désactivez-le plutôt, l\'historique doit rester intact'
                      : 'Supprimer'"
                    @click="retirer(c)"
                  >
                    <font-awesome-icon icon="fa-solid fa-trash" />
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- ─── Paramètres de monétisation ─── -->
      <section v-if="parametres" class="space-y-3">
        <h2 class="text-lg font-semibold">Paramètres de monétisation</h2>

        <div class="alert alert-warning py-2 text-sm">
          <font-awesome-icon icon="fa-solid fa-triangle-exclamation" />
          <span>
            Tant que <strong>« paiement réel »</strong> est désactivé, tout paiement est
            <strong>simulé</strong> : les membres obtiennent gratuitement points et cagnottes.
            C'est un risque accepté le temps de la phase de test, et la
            <NuxtLink to="/admin/engagement/transactions" class="link">purge de fin de phase</NuxtLink>
            en est la contrepartie — elle n'est d'ailleurs accessible qu'une fois ce drapeau activé.
          </span>
        </div>

        <div class="card bg-base-200">
          <div class="card-body gap-3">
            <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
              <label class="form-control">
                <span class="label-text text-xs">Taux de commission (%)</span>
                <input
                  v-model.number="parametres.taux_commission"
                  type="number" min="0" max="100"
                  class="input input-sm input-bordered"
                >
                <span class="label-text-alt mt-1 text-xs opacity-60">
                  Part revenant à la plateforme sur un cadeau « soutien financier ».
                  Prospectif : l'historique conserve son taux.
                </span>
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Devise (code ISO)</span>
                <input v-model="parametres.devise" maxlength="3" class="input input-sm input-bordered font-mono uppercase">
              </label>
              <label class="form-control">
                <span class="label-text text-xs">Paiement réel activé</span>
                <input v-model="parametres.paiement_reel_actif" type="checkbox" class="toggle toggle-sm mt-1">
                <span class="label-text-alt mt-1 text-xs opacity-60">
                  Masque le bandeau « phase de test » côté membre et débloque la purge.
                </span>
              </label>
            </div>
            <div class="card-actions justify-end">
              <button class="btn btn-sm btn-primary" @click="enregistrerParametres">
                Enregistrer les paramètres
              </button>
            </div>
          </div>
        </div>
      </section>
    </template>
  </div>
</template>
